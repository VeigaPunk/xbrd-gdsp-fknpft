use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::fs::OpenOptions;
use std::io::Write;
use std::path::Path;
use std::time::{SystemTime, UNIX_EPOCH};

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
        if !name_str.starts_with(&sidecar_prefix) || name_str.ends_with(".tmp") {
            continue;
        }
        let sidecar = entry.path();
        let drain_sidecar = sidecar.with_extension(format!(
            "drained_by.{}.{}",
            std::process::id(),
            name_str.trim_start_matches(&sidecar_prefix)
        ));
        // Claim exclusive ownership via rename; another drain may have grabbed
        // it between the readdir and this rename — that's fine, skip silently.
        if std::fs::rename(&sidecar, &drain_sidecar).is_err() {
            continue;
        }
        match std::fs::read_to_string(&drain_sidecar) {
            Ok(c) => collected.push_str(&c),
            Err(err) => eprintln!("xbreed mailbox: sidecar read failed: {err}"),
        }
        let _ = std::fs::remove_file(&drain_sidecar);
    }
    Ok(collected)
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
pub fn compact_events(
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
        // keep_types=concern: concern events survive verbatim; keepalive is compacted
        let keep_types = vec!["concern".to_string()];
        let (kept, compacted) = compact_events(dir.path(), &keep_types, 1).unwrap();
        assert_eq!(compacted, 1, "one keepalive should be compacted");
        assert_eq!(kept, 3, "2 concerns + 1 digest");
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
        // no keep_types, age cutoff=1s: both old events are compactable
        let (kept, compacted) = compact_events(dir.path(), &[], 1).unwrap();
        assert_eq!(compacted, 2);
        assert_eq!(kept, 1, "only the digest event");
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
        // After compact, the live mailbox path does not exist until the next
        // write_event call. Drain must still return the compact output.
        let dir = tempdir().unwrap();
        write_old_event(dir.path(), "x", "keepalive", "old");
        write_old_event(dir.path(), "y", "keepalive", "older");
        let (kept, compacted) = compact_events(dir.path(), &[], 1).unwrap();
        assert_eq!((kept, compacted), (1, 2));

        let path = mailbox_path(dir.path());
        assert!(!path.exists(), "compact should not recreate live mailbox");

        let events = drain_events(dir.path()).unwrap();
        assert_eq!(events.len(), 1);
        assert_eq!(events[0].event_type, "digest");

        // Second drain returns empty — sidecar consumed.
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
        assert_eq!(total, 10, "sidecar must be consumed exactly once: got {total}");
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
        assert!(tmp_sidecar.exists(), ".tmp sidecar must not be consumed by drain");
    }
}
