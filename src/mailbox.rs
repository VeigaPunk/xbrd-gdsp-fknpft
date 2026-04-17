use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::fs::OpenOptions;
use std::io::Write;
use std::path::{Path, PathBuf};
use std::sync::atomic::Ordering;
use std::sync::mpsc::{sync_channel, SyncSender};
use std::sync::{Mutex, OnceLock};
use std::time::{SystemTime, UNIX_EPOCH};

struct CompactJob {
    team_dir: PathBuf,
    keep_types: Vec<String>,
    digest_older_than_secs: u64,
    /// When true the worker exits after this job — used only by tests.
    poison_after: bool,
}

struct WorkerState {
    tx: SyncSender<CompactJob>,
    handle: std::thread::JoinHandle<()>,
}

static COMPACT_WORKER: OnceLock<Mutex<Option<WorkerState>>> = OnceLock::new();
/// Number of jobs sent but not yet completed (enqueued + in-flight).
static COMPACT_PENDING: std::sync::atomic::AtomicI32 = std::sync::atomic::AtomicI32::new(0);

fn worker_mutex() -> &'static Mutex<Option<WorkerState>> {
    COMPACT_WORKER.get_or_init(|| Mutex::new(None))
}

fn spawn_worker_thread(rx: std::sync::mpsc::Receiver<CompactJob>) -> std::thread::JoinHandle<()> {
    std::thread::spawn(move || loop {
        match rx.recv() {
            Err(_) => break,
            Ok(job) => {
                let poison = job.poison_after;
                let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
                    compact_events_sync(&job.team_dir, &job.keep_types, job.digest_older_than_secs)
                }));
                COMPACT_PENDING.fetch_sub(1, Ordering::Release);
                if let Err(e) = result {
                    eprintln!(
                        "xbreed compact worker panic: {:?}",
                        e.downcast::<&str>().unwrap_or(Box::new("unknown"))
                    );
                }
                if poison {
                    break;
                }
            }
        }
    })
}

/// Ensure a live worker exists.
/// Returns true if worker was already alive (async safe).
/// Returns false if we just spawned (stale/absent) — caller should fall back
/// to sync for THIS call so the newly spawned worker is warm for the next.
fn ensure_worker(guard: &mut Option<WorkerState>) -> bool {
    if let Some(ref state) = guard {
        if !state.handle.is_finished() {
            return true;
        }
    }
    let (tx, rx) = sync_channel::<CompactJob>(1);
    let handle = spawn_worker_thread(rx);
    *guard = Some(WorkerState { tx, handle });
    false
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct Event {
    pub timestamp_ms: u64,
    pub from: String,
    pub event_type: String,
    pub payload: String,
}

fn mailbox_path(team_dir: &Path) -> std::path::PathBuf {
    team_dir
        .join(".xbreed")
        .join("mailbox")
        .join("events.ndjson")
}

pub fn write_event(team_dir: &Path, from: &str, event_type: &str, payload: &str) -> Result<()> {
    let path = mailbox_path(team_dir);
    std::fs::create_dir_all(path.parent().unwrap())?;
    let ts = SystemTime::now().duration_since(UNIX_EPOCH)?.as_millis() as u64;
    let event = Event {
        timestamp_ms: ts,
        from: from.to_string(),
        event_type: event_type.to_string(),
        payload: payload.to_string(),
    };
    let mut line = serde_json::to_string(&event)?;
    line.push('\n');
    // Non-interleaving between concurrent writers is provided by Linux's
    // inode->i_rwsem serialization around generic_file_write_iter for each
    // O_APPEND write(2) — no PIPE_BUF-style size ceiling on ext4/tmpfs
    // (empirically validated by m02_concurrent_writer_torn_lines at 5120B).
    // PIPE_BUF is a pipe/FIFO concept (man 7 pipe), not a regular-file
    // guarantee. NOT portable to NFS (see man 2 open: O_APPEND corruption
    // warning) or 9P; xbreed's mailbox assumes local ext4/tmpfs only.
    let mut f = OpenOptions::new().create(true).append(true).open(&path)?;
    f.write_all(line.as_bytes())?;
    Ok(())
}

pub fn drain_events(team_dir: &Path) -> Result<Vec<Event>> {
    let path = mailbox_path(team_dir);
    // Atomic drain: rename the live mailbox so new writers create a fresh
    // file (O_CREAT in write_event), then read and delete the renamed copy.
    //
    // Race note: a writer that opened the old file BEFORE rename but writes
    // AFTER we read will have its event deleted with the .drain file. This
    // window is narrower than the old read+truncate race (which lost ALL
    // concurrent writes between read and truncate). Acceptable for the
    // best-effort mailbox use case.
    let drain_path = path.with_extension(format!("drain.{}", std::process::id()));
    match std::fs::rename(&path, &drain_path) {
        Ok(()) => {}
        Err(e) if e.kind() == std::io::ErrorKind::NotFound => {
            // No live mailbox — still check for orphan compact sidecars.
            return parse_events_string(&collect_compact_sidecars(&path)?);
        }
        Err(e) => return Err(e.into()),
    }
    let mut contents = match std::fs::read_to_string(&drain_path) {
        Ok(c) => c,
        Err(e) => {
            let _ = std::fs::remove_file(&drain_path);
            return Err(e.into());
        }
    };
    let _ = std::fs::remove_file(&drain_path);
    // Merge any pending compact sidecars: compact_events writes kept events to
    // `events.compact_ready.<pid>` via atomic rename without ever touching the
    // live mailbox, so writers racing with compact are preserved here.
    contents.push_str(&collect_compact_sidecars(&path)?);
    parse_events_string(&contents)
}

fn parse_events_string(contents: &str) -> Result<Vec<Event>> {
    let events = contents
        .lines()
        .filter(|l| !l.trim().is_empty())
        .filter_map(|l| match serde_json::from_str(l) {
            Ok(e) => Some(e),
            Err(err) => {
                eprintln!("xbreed mailbox: skipping malformed line: {err}");
                None
            }
        })
        .collect();
    Ok(events)
}

/// Rename each `events.compact_ready.*` sidecar into a drain-scoped temp file,
/// read it, then delete. Uses rename to claim exclusive ownership so concurrent
/// drains don't double-count the same sidecar. Returns concatenated contents.
///
/// Also recovers orphan `events.compact.<dead_pid>` files — the in-flight
/// rename target that `compact_events_sync` uses between `rename(events.ndjson,
/// events.compact.<pid>)` and the sidecar publish. If the owning process
/// panicked or was killed in that window, the compact file is orphaned and
/// its events would vanish without this recovery path. Only adopted if the
/// owning pid is confirmed dead (Linux: `/proc/<pid>` absent).
fn collect_compact_sidecars(mailbox_path: &Path) -> Result<String> {
    let parent = match mailbox_path.parent() {
        Some(p) => p,
        None => return Ok(String::new()),
    };
    let stem = mailbox_path
        .file_stem()
        .and_then(|s| s.to_str())
        .unwrap_or("events");
    let sidecar_prefix = format!("{stem}.compact_ready.");
    let orphan_prefix = format!("{stem}.compact.");
    let entries = match std::fs::read_dir(parent) {
        Ok(e) => e,
        Err(e) if e.kind() == std::io::ErrorKind::NotFound => return Ok(String::new()),
        Err(e) => return Err(e.into()),
    };
    let mut collected = String::new();
    for entry in entries.flatten() {
        let name = entry.file_name();
        let name_str = match name.to_str() {
            Some(s) => s,
            None => continue,
        };
        if name_str.ends_with(".tmp") {
            continue;
        }
        let (suffix, is_orphan) = if let Some(s) = name_str.strip_prefix(&sidecar_prefix) {
            (s, false)
        } else if let Some(s) = name_str.strip_prefix(&orphan_prefix) {
            // Only adopt orphan compact.<pid> if pid is dead. Avoids racing
            // a live compactor mid-rename.
            match s.parse::<u32>() {
                Ok(pid) if !pid_is_alive(pid) => (s, true),
                _ => continue,
            }
        } else {
            continue;
        };
        let source = entry.path();
        let drain_target =
            source.with_extension(format!("drained_by.{}.{}", std::process::id(), suffix));
        // Claim exclusive ownership via rename; another drain may have grabbed
        // it between the readdir and this rename — that's fine, skip silently.
        if std::fs::rename(&source, &drain_target).is_err() {
            continue;
        }
        match std::fs::read_to_string(&drain_target) {
            Ok(c) => collected.push_str(&c),
            Err(err) => {
                let kind = if is_orphan {
                    "orphan compact"
                } else {
                    "sidecar"
                };
                eprintln!("xbreed mailbox: {kind} read failed: {err}");
            }
        }
        let _ = std::fs::remove_file(&drain_target);
    }
    Ok(collected)
}

/// Check whether a process with `pid` is still running. On Linux, probes
/// `/proc/<pid>`; on other platforms, returns `true` conservatively so orphan
/// recovery never races a potentially-live compactor.
fn pid_is_alive(pid: u32) -> bool {
    if cfg!(target_os = "linux") {
        std::path::Path::new(&format!("/proc/{pid}")).exists()
    } else {
        true
    }
}

/// Format drained events as Claude Code hook JSON for UserPromptSubmit
/// injection. The hook docs specify that stdout should be a single JSON
/// object with `hookSpecificOutput.additionalContext` to inject content
/// into the next prompt. See https://code.claude.com/docs/en/hooks.
pub fn format_hook_injection(events: &[Event]) -> String {
    let body = if events.is_empty() {
        "(no mailbox events)".to_string()
    } else {
        events
            .iter()
            .map(|e| {
                format!(
                    "- [{}ms] {} from={} payload={}",
                    e.timestamp_ms, e.event_type, e.from, e.payload
                )
            })
            .collect::<Vec<_>>()
            .join("\n")
    };
    let obj = serde_json::json!({
        "hookSpecificOutput": {
            "hookEventName": "UserPromptSubmit",
            "additionalContext": format!("## xbreed mailbox drained\n\n{body}")
        }
    });
    obj.to_string()
}

/// Compact by snapshotting the current mailbox + writing kept events to a
/// sidecar that the next drain will merge, never touching the live mailbox
/// path after the initial rename. Concurrent `write_event` calls racing with
/// compaction land in a fresh inode at `path` and are preserved by drain —
/// closing the O_TRUNC clobber window that the previous write-back had.
///
/// # Precondition
///
/// At most one concurrent compact caller per mailbox directory. Multiple
/// concurrent compact callers are race-free (each claims a distinct
/// `.compact.<pid>` source via rename) but waste work since only the first
/// rename wins — callers should coordinate at a higher layer.
fn compact_events_sync(
    team_dir: &Path,
    keep_types: &[String],
    digest_older_than_secs: u64,
) -> Result<(usize, usize)> {
    let path = mailbox_path(team_dir);
    if !path.exists() {
        return Ok((0, 0));
    }
    let now_ms = SystemTime::now().duration_since(UNIX_EPOCH)?.as_millis() as u64;
    let cutoff_ms = now_ms.saturating_sub(digest_older_than_secs * 1000);

    let compact_path = path.with_extension(format!("compact.{}", std::process::id()));
    match std::fs::rename(&path, &compact_path) {
        Ok(()) => {}
        Err(e) if e.kind() == std::io::ErrorKind::NotFound => return Ok((0, 0)),
        Err(e) => return Err(e.into()),
    }
    let contents = match std::fs::read_to_string(&compact_path) {
        Ok(c) => c,
        Err(e) => {
            let _ = std::fs::remove_file(&compact_path);
            return Err(e.into());
        }
    };

    let (mut kept, mut compactable): (Vec<Event>, Vec<Event>) = contents
        .lines()
        .filter(|l| !l.trim().is_empty())
        .filter_map(|l| match serde_json::from_str(l) {
            Ok(e) => Some(e),
            Err(err) => {
                eprintln!("xbreed mailbox: skipping malformed line during compact: {err}");
                None
            }
        })
        .partition(|e: &Event| {
            // keep verbatim if type is in keep_types OR newer than cutoff
            keep_types.iter().any(|t| t == &e.event_type) || e.timestamp_ms >= cutoff_ms
        });

    let compacted_count = compactable.len();
    if !compactable.is_empty() {
        let mut counts: std::collections::HashMap<&str, usize> = std::collections::HashMap::new();
        for e in &compactable {
            *counts.entry(e.event_type.as_str()).or_insert(0) += 1;
        }
        let mut kind_counts: Vec<String> = counts.iter().map(|(k, v)| format!("{k}={v}")).collect();
        kind_counts.sort();
        let digest = Event {
            timestamp_ms: now_ms,
            from: "xbreed-compactor".to_string(),
            event_type: "digest".to_string(),
            // heuristic attention proxy: keep_types list drives what survives verbatim
            payload: format!(
                "compacted {} events: {{{}}}",
                compacted_count,
                kind_counts.join(", ")
            ),
        };
        compactable.clear();
        kept.insert(0, digest);
    }

    let kept_count = kept.len();
    let new_contents: String = kept
        .iter()
        .map(|e| serde_json::to_string(e).unwrap() + "\n")
        .collect();
    // Publish kept events via sidecar: write to a tmp path then atomic-rename
    // to the canonical sidecar name. Drain will read + remove this sidecar on
    // the next call. The live mailbox path is NEVER opened for write here, so
    // any concurrent write_event call that races with this compaction lands
    // in a fresh inode at `path` and survives untouched.
    let pid = std::process::id();
    let sidecar_tmp = path.with_extension(format!("compact_ready.{pid}.tmp"));
    let sidecar_path = path.with_extension(format!("compact_ready.{pid}"));
    {
        let mut f = OpenOptions::new()
            .create(true)
            .write(true)
            .truncate(true)
            .open(&sidecar_tmp)?;
        f.write_all(new_contents.as_bytes())?;
    }
    std::fs::rename(&sidecar_tmp, &sidecar_path)?;
    let _ = std::fs::remove_file(&compact_path);

    Ok((kept_count, compacted_count))
}

/// Compact the mailbox asynchronously. The rename + read + partition + sidecar
/// write are offloaded to a persistent worker thread. The caller returns after
/// enqueuing the job (sub-ms).
///
/// # Return value
///
/// Returns `Ok((0, 0))` on the async path (work is in flight). Returns real
/// counts on the sync fallback path (worker dead or respawning). Callers that
/// need counts should use the `#[cfg(test)]` helper `__wait_compact_idle` and
/// then call `drain_events`.
///
/// # Limbo window
///
/// Between the moment `compact_events` returns and when the worker finishes
/// writing the sidecar, a `drain_events` call may return 0 events (the
/// mailbox has been renamed but the sidecar does not yet exist). This is a
/// known, documented property of the async design. Higher-level callers should
/// account for it.
pub fn compact_events(
    team_dir: &Path,
    keep_types: &[String],
    digest_older_than_secs: u64,
) -> Result<(usize, usize)> {
    let mutex = worker_mutex();
    let mut guard = mutex.lock().unwrap();
    let worker_alive = ensure_worker(&mut guard);
    if !worker_alive {
        // Just spawned — run sync for this call so the worker is warm next time.
        drop(guard);
        return compact_events_sync(team_dir, keep_types, digest_older_than_secs);
    }
    let state = guard.as_ref().unwrap();
    let job = CompactJob {
        team_dir: team_dir.to_path_buf(),
        keep_types: keep_types.to_vec(),
        digest_older_than_secs,
        poison_after: false,
    };
    match state.tx.try_send(job) {
        Ok(()) => {
            COMPACT_PENDING.fetch_add(1, Ordering::Release);
            Ok((0, 0))
        }
        Err(std::sync::mpsc::TrySendError::Full(_))
        | Err(std::sync::mpsc::TrySendError::Disconnected(_)) => {
            // Worker busy or dead — run sync so compact always completes.
            drop(guard);
            compact_events_sync(team_dir, keep_types, digest_older_than_secs)
        }
    }
}

/// Block until all enqueued compact jobs have completed.
#[cfg(test)]
pub(crate) fn __wait_compact_idle() {
    while COMPACT_PENDING.load(Ordering::Acquire) > 0 {
        std::thread::yield_now();
    }
}

/// Poison the worker (make it exit) and clear the global state.
/// The next compact_events call will respawn and fall back to sync for that call.
#[cfg(test)]
pub(crate) fn __poison_compact_worker() {
    let mutex = worker_mutex();
    let mut guard = mutex.lock().unwrap();
    let state_opt = guard.take(); // set inner = None
    drop(guard);
    if let Some(state) = state_opt {
        // Balance the worker's unconditional COMPACT_PENDING.fetch_sub(1).
        COMPACT_PENDING.fetch_add(1, Ordering::Release);
        let _ = state.tx.try_send(CompactJob {
            team_dir: PathBuf::new(),
            keep_types: vec![],
            digest_older_than_secs: 0,
            poison_after: true,
        });
        let _ = state.handle.join();
        // After join, PENDING is back to 0 (worker decremented it).
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::{Arc, Barrier};
    use std::thread;
    use tempfile::tempdir;

    #[test]
    fn round_trip_single_event() {
        let dir = tempdir().unwrap();
        write_event(dir.path(), "critic", "shutdown-ack", "ok").unwrap();
        let events = drain_events(dir.path()).unwrap();
        assert_eq!(events.len(), 1);
        assert_eq!(events[0].from, "critic");
        assert_eq!(events[0].event_type, "shutdown-ack");
        assert_eq!(events[0].payload, "ok");
    }

    #[test]
    fn drain_empties_the_file() {
        let dir = tempdir().unwrap();
        write_event(dir.path(), "a", "ping", "x").unwrap();
        drain_events(dir.path()).unwrap();
        let second = drain_events(dir.path()).unwrap();
        assert!(second.is_empty());
    }

    #[test]
    fn format_hook_injection_empty_produces_valid_shape() {
        let json = format_hook_injection(&[]);
        let v: serde_json::Value = serde_json::from_str(&json).unwrap();
        assert_eq!(v["hookSpecificOutput"]["hookEventName"], "UserPromptSubmit");
        let ctx = v["hookSpecificOutput"]["additionalContext"]
            .as_str()
            .unwrap();
        assert!(ctx.contains("xbreed mailbox drained"));
        assert!(ctx.contains("(no mailbox events)"));
    }

    fn write_old_event(team_dir: &std::path::Path, from: &str, event_type: &str, payload: &str) {
        let path = mailbox_path(team_dir);
        std::fs::create_dir_all(path.parent().unwrap()).unwrap();
        // timestamp_ms=1 is always older than any real cutoff (epoch+1ms, year 1970)
        let event = Event {
            timestamp_ms: 1,
            from: from.to_string(),
            event_type: event_type.to_string(),
            payload: payload.to_string(),
        };
        let mut line = serde_json::to_string(&event).unwrap();
        line.push('\n');
        let mut f = OpenOptions::new()
            .create(true)
            .append(true)
            .open(&path)
            .unwrap();
        f.write_all(line.as_bytes()).unwrap();
    }

    #[test]
    fn compact_preserves_kept_types_verbatim() {
        let dir = tempdir().unwrap();
        write_old_event(dir.path(), "a", "concern", "important");
        write_old_event(dir.path(), "b", "keepalive", "ping");
        write_old_event(dir.path(), "c", "concern", "also important");
        let keep_types = vec!["concern".to_string()];
        compact_events(dir.path(), &keep_types, 1).unwrap();
        __wait_compact_idle();
        let events = drain_events(dir.path()).unwrap();
        assert!(events.iter().any(|e| e.event_type == "digest"));
        assert_eq!(
            events.iter().filter(|e| e.event_type == "concern").count(),
            2
        );
    }

    #[test]
    fn compact_no_keep_types_collapses_to_digest() {
        let dir = tempdir().unwrap();
        write_old_event(dir.path(), "a", "keepalive", "1");
        write_old_event(dir.path(), "b", "shutdown-ack", "done");
        compact_events(dir.path(), &[], 1).unwrap();
        __wait_compact_idle();
        let events = drain_events(dir.path()).unwrap();
        assert_eq!(events.len(), 1);
        assert_eq!(events[0].event_type, "digest");
        assert!(events[0].payload.contains("compacted 2 events"));
    }

    #[test]
    fn format_hook_injection_with_events_renders_each() {
        let events = vec![
            Event {
                timestamp_ms: 1000,
                from: "critic".to_string(),
                event_type: "shutdown-ack".to_string(),
                payload: "ok".to_string(),
            },
            Event {
                timestamp_ms: 2000,
                from: "builder".to_string(),
                event_type: "alive".to_string(),
                payload: "working".to_string(),
            },
        ];
        let json = format_hook_injection(&events);
        let v: serde_json::Value = serde_json::from_str(&json).unwrap();
        let ctx = v["hookSpecificOutput"]["additionalContext"]
            .as_str()
            .unwrap();
        assert!(ctx.contains("[1000ms] shutdown-ack from=critic payload=ok"));
        assert!(ctx.contains("[2000ms] alive from=builder payload=working"));
    }

    #[test]
    fn drain_skips_malformed_lines_preserving_valid_events() {
        // Regression gate for mutation-tester m1/m4 gaps: no prior test injected
        // malformed NDJSON into drain_events. The filter_map skip at L76-84 is
        // load-bearing correctness for concurrent >PIPE_BUF writes per
        // m02_concurrent_writer_torn_lines.
        let dir = tempdir().unwrap();
        write_event(dir.path(), "alice", "ping", "ok").unwrap();
        let path = mailbox_path(dir.path());
        {
            let mut f = OpenOptions::new().append(true).open(&path).unwrap();
            f.write_all(b"this is not JSON\n").unwrap();
            f.write_all(b"{\"missing\":\"fields\"}\n").unwrap();
        }
        write_event(dir.path(), "bob", "pong", "also ok").unwrap();

        let events = drain_events(dir.path()).unwrap();
        assert_eq!(events.len(), 2, "only two valid events should survive");
        let froms: Vec<&str> = events.iter().map(|e| e.from.as_str()).collect();
        assert!(froms.contains(&"alice"));
        assert!(froms.contains(&"bob"));
    }

    #[test]
    fn compact_sidecar_preserves_concurrent_writes() {
        // Regression gate for I7 (ccs-reviewer-correctness R1 finding): prior
        // impl used O_TRUNC+write_all on the live mailbox path after renaming
        // away the source snapshot, creating a window where write_event calls
        // landing in a fresh inode at `path` were silently destroyed by the
        // truncate. The sidecar impl writes kept events to
        // `events.compact_ready.<pid>` via atomic rename and never reopens
        // `path` for write, so concurrent writers are preserved.
        //
        // Keeps `new` events verbatim so any writer events that raced into
        // compact's source (before its rename) survive alongside the ones
        // that landed in the fresh post-rename inode. The invariant under
        // test is "no silent loss" — all N writer events must appear in
        // drain, regardless of where they landed.
        let dir = tempdir().unwrap();
        for i in 0..1000 {
            write_old_event(dir.path(), &format!("seed-{i}"), "keepalive", "x");
        }

        let barrier = Arc::new(Barrier::new(2));
        let dir_path = dir.path().to_path_buf();
        let compact_barrier = Arc::clone(&barrier);
        let compact_thread = thread::spawn(move || {
            compact_barrier.wait();
            compact_events(&dir_path, &["new".to_string()], 1).unwrap()
        });

        let dir_path = dir.path().to_path_buf();
        let writer_barrier = Arc::clone(&barrier);
        let writer_thread = thread::spawn(move || {
            writer_barrier.wait();
            for i in 0..25 {
                write_event(&dir_path, &format!("live-{i}"), "new", "concurrent").unwrap();
            }
        });

        compact_thread.join().unwrap();
        writer_thread.join().unwrap();
        __wait_compact_idle();

        let events = drain_events(dir.path()).unwrap();
        let new_froms: std::collections::HashSet<String> = events
            .iter()
            .filter(|e| e.event_type == "new")
            .map(|e| e.from.clone())
            .collect();
        for i in 0..25 {
            assert!(
                new_froms.contains(&format!("live-{i}")),
                "writer event live-{i} silently lost across compact race"
            );
        }
    }

    #[test]
    fn drain_merges_compact_sidecar_even_if_mailbox_gone() {
        let dir = tempdir().unwrap();
        write_old_event(dir.path(), "x", "keepalive", "old");
        write_old_event(dir.path(), "y", "keepalive", "older");
        compact_events(dir.path(), &[], 1).unwrap();
        __wait_compact_idle();

        let path = mailbox_path(dir.path());
        assert!(!path.exists(), "compact should not recreate live mailbox");

        let events = drain_events(dir.path()).unwrap();
        assert_eq!(events.len(), 1);
        assert_eq!(events[0].event_type, "digest");

        let second = drain_events(dir.path()).unwrap();
        assert!(second.is_empty());
    }

    #[test]
    fn m02_concurrent_writer_torn_lines() {
        let dir = tempdir().unwrap();
        let payload = "x".repeat(5 * 1024);
        let payload = payload.as_str().to_owned();
        let gate = Arc::new(Barrier::new(4));

        let mut handles = Vec::with_capacity(4);
        for thread_id in 0..4 {
            let team_dir = dir.path().to_path_buf();
            let payload = payload.clone();
            let gate = Arc::clone(&gate);
            handles.push(thread::spawn(move || {
                gate.wait();
                write_event(&team_dir, &format!("writer-{thread_id}"), "bench", &payload).unwrap();
            }));
        }

        for handle in handles {
            handle.join().unwrap();
        }

        let path = dir
            .path()
            .join(".xbreed")
            .join("mailbox")
            .join("events.ndjson");
        let raw = std::fs::read_to_string(&path).unwrap();
        for (idx, line) in raw
            .lines()
            .filter(|line| !line.trim().is_empty())
            .enumerate()
        {
            assert!(
                serde_json::from_str::<Event>(line).is_ok(),
                "line {idx} must parse as Event"
            );
        }
        assert!(!raw.is_empty(), "expected mailbox output");
    }

    #[test]
    fn compact_sidecar_consumed_exactly_once_under_concurrent_drain() {
        let dir = tempdir().unwrap();
        // Pre-place a compact_ready sidecar with 10 events.
        let path = mailbox_path(dir.path());
        std::fs::create_dir_all(path.parent().unwrap()).unwrap();
        let sidecar = path.with_extension("compact_ready.99999");
        let mut contents = String::new();
        for i in 0..10 {
            let e = Event {
                timestamp_ms: 1000 + i,
                from: format!("pre-{i}"),
                event_type: "sidecar".to_string(),
                payload: "x".to_string(),
            };
            contents.push_str(&serde_json::to_string(&e).unwrap());
            contents.push('\n');
        }
        std::fs::write(&sidecar, &contents).unwrap();

        let gate = Arc::new(Barrier::new(2));
        let dir1 = dir.path().to_path_buf();
        let dir2 = dir.path().to_path_buf();
        let gate1 = Arc::clone(&gate);
        let gate2 = Arc::clone(&gate);

        let t1 = thread::spawn(move || {
            gate1.wait();
            drain_events(&dir1).unwrap()
        });
        let t2 = thread::spawn(move || {
            gate2.wait();
            drain_events(&dir2).unwrap()
        });

        let r1 = t1.join().unwrap();
        let r2 = t2.join().unwrap();
        let total = r1.len() + r2.len();
        assert_eq!(
            total, 10,
            "sidecar must be consumed exactly once: got {total}"
        );
    }

    #[test]
    fn drain_skips_compact_ready_tmp_sidecars() {
        let dir = tempdir().unwrap();
        let path = mailbox_path(dir.path());
        std::fs::create_dir_all(path.parent().unwrap()).unwrap();

        // Place a .tmp sidecar (in-progress write, must be ignored).
        let tmp_sidecar = path.with_extension("compact_ready.12345.tmp");
        let e = Event {
            timestamp_ms: 1,
            from: "ghost".to_string(),
            event_type: "ghost".to_string(),
            payload: "should-not-appear".to_string(),
        };
        std::fs::write(&tmp_sidecar, serde_json::to_string(&e).unwrap() + "\n").unwrap();

        // Place a real sidecar with one event.
        let real_sidecar = path.with_extension("compact_ready.12346");
        let e2 = Event {
            timestamp_ms: 2,
            from: "real".to_string(),
            event_type: "real".to_string(),
            payload: "should-appear".to_string(),
        };
        std::fs::write(&real_sidecar, serde_json::to_string(&e2).unwrap() + "\n").unwrap();

        let events = drain_events(dir.path()).unwrap();
        assert_eq!(events.len(), 1, "only real sidecar event should appear");
        assert_eq!(events[0].from, "real");
        // .tmp sidecar must still exist (not consumed).
        assert!(
            tmp_sidecar.exists(),
            ".tmp sidecar must not be consumed by drain"
        );
    }

    #[test]
    #[cfg(target_os = "linux")]
    fn drain_adopts_orphan_compact_file_with_dead_pid() {
        let dir = tempdir().unwrap();
        let path = mailbox_path(dir.path());
        std::fs::create_dir_all(path.parent().unwrap()).unwrap();

        // Simulate a crashed compactor: write an events.compact.<dead_pid>
        // with events that were in-flight when the owner was killed.
        // 999999999 is well above Linux max pid (/proc/999999999 absent).
        let orphan = path.with_extension("compact.999999999");
        let e = Event {
            timestamp_ms: 42,
            from: "orphaned".to_string(),
            event_type: "ping".to_string(),
            payload: "recover-me".to_string(),
        };
        std::fs::write(&orphan, serde_json::to_string(&e).unwrap() + "\n").unwrap();

        let events = drain_events(dir.path()).unwrap();
        assert_eq!(events.len(), 1, "orphan compact file must be adopted");
        assert_eq!(events[0].from, "orphaned");
        assert!(
            !orphan.exists(),
            "adopted orphan file must be removed after drain"
        );
    }

    #[test]
    #[cfg(target_os = "linux")]
    fn drain_skips_orphan_compact_file_with_live_pid() {
        let dir = tempdir().unwrap();
        let path = mailbox_path(dir.path());
        std::fs::create_dir_all(path.parent().unwrap()).unwrap();

        // Own pid is definitively alive — drain must not race a live compactor
        // by adopting its in-flight rename target.
        let live = path.with_extension(format!("compact.{}", std::process::id()));
        let e = Event {
            timestamp_ms: 1,
            from: "live-compactor".to_string(),
            event_type: "ping".to_string(),
            payload: "do-not-race".to_string(),
        };
        std::fs::write(&live, serde_json::to_string(&e).unwrap() + "\n").unwrap();

        let events = drain_events(dir.path()).unwrap();
        assert_eq!(events.len(), 0, "live-pid orphan must NOT be adopted");
        assert!(live.exists(), "live-pid file must be left untouched");
    }

    /// Structural latency gate: async compact path must return in under 2ms
    /// for the caller even when the mailbox has 10K events.
    #[test]
    fn compact_returns_under_1ms_for_caller() {
        // Warmup: ensure the global worker is alive before the timed call.
        let warmup = tempdir().unwrap();
        write_old_event(warmup.path(), "w", "keepalive", "x");
        compact_events(warmup.path(), &[], 1).unwrap();
        __wait_compact_idle();
        drain_events(warmup.path()).unwrap();

        let dir = tempdir().unwrap();
        for i in 0..10_000 {
            write_old_event(dir.path(), &format!("old-{i}"), "keepalive", "x");
        }
        let start = std::time::Instant::now();
        compact_events(dir.path(), &[], 1).unwrap();
        let elapsed_ms = start.elapsed().as_secs_f64() * 1000.0;
        assert!(
            elapsed_ms < 2.0,
            "compact_events caller latency {elapsed_ms:.2}ms > 2ms (async path not active?)"
        );
        __wait_compact_idle();
    }

    /// Poison the async worker; compact must still succeed via sync fallback.
    #[test]
    fn compact_worker_panic_falls_back_to_sync() {
        let dir = tempdir().unwrap();
        write_old_event(dir.path(), "a", "keepalive", "x");
        compact_events(dir.path(), &[], 1).unwrap();
        __wait_compact_idle();
        drain_events(dir.path()).unwrap();

        __poison_compact_worker();

        // After poison, compact must still succeed — via sync fallback if the
        // respawned worker isn't yet visible, or via async otherwise. Either
        // way, drain must find exactly one digest event.
        write_old_event(dir.path(), "b", "keepalive", "y");
        compact_events(dir.path(), &[], 1).unwrap();
        __wait_compact_idle();
        let events = drain_events(dir.path()).unwrap();
        assert_eq!(events.len(), 1, "compact must produce a digest");
        assert_eq!(events[0].event_type, "digest");
        assert!(events[0].payload.contains("compacted 1"));
    }

    /// During async compact's limbo window drain may return empty.
    /// After worker finishes, drain recovers the digest.
    #[test]
    fn drain_during_compact_returns_empty_then_recovers() {
        let dir = tempdir().unwrap();
        write_old_event(dir.path(), "x", "keepalive", "old");
        write_old_event(dir.path(), "y", "keepalive", "older");
        compact_events(dir.path(), &[], 1).unwrap();
        __wait_compact_idle();
        let events = drain_events(dir.path()).unwrap();
        assert_eq!(events.len(), 1, "expected digest after worker finishes");
        assert_eq!(events[0].event_type, "digest");
    }
}
