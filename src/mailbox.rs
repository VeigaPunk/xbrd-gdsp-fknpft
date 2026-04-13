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
    // append is atomic on Linux for writes < PIPE_BUF (4096 bytes)
    let mut f = OpenOptions::new().create(true).append(true).open(&path)?;
    f.write_all(line.as_bytes())?;
    Ok(())
}

pub fn drain_events(team_dir: &Path) -> Result<Vec<Event>> {
    let path = mailbox_path(team_dir);
    // Atomic drain: rename the mailbox file so new writers create a fresh
    // file (O_CREAT in write_event), then read and delete the renamed copy.
    //
    // Race note: a writer that opened the old file BEFORE rename but writes
    // AFTER we read will have its event deleted with the .drain file. This
    // window is narrower than the old read+truncate race (which lost ALL
    // concurrent writes between read and truncate). Acceptable for the
    // best-effort mailbox use case.
    let drain_path = path.with_extension(format!("drain.{}", std::process::id()));
    // Recover a leaked .drain file from a prior failed drain (same PID is
    // impossible, but a stale drain.<other-pid> can exist). We don't recover
    // those — they're from a dead process and will be overwritten if the same
    // PID is recycled. Acceptable for best-effort mailbox.
    match std::fs::rename(&path, &drain_path) {
        Ok(()) => {}
        Err(e) if e.kind() == std::io::ErrorKind::NotFound => return Ok(vec![]),
        Err(e) => return Err(e.into()),
    }
    // Always clean up the drain file, even if read fails.
    let contents = match std::fs::read_to_string(&drain_path) {
        Ok(c) => c,
        Err(e) => {
            let _ = std::fs::remove_file(&drain_path);
            return Err(e.into());
        }
    };
    let _ = std::fs::remove_file(&drain_path);
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

    // Atomic compact: rename → read → process → write new file → delete old.
    // Same pattern as drain_events to avoid the read+truncate race.
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
    // Write compacted events to a fresh mailbox file, then clean up.
    let mut f = OpenOptions::new()
        .create(true)
        .write(true)
        .truncate(true)
        .open(&path)?;
    f.write_all(new_contents.as_bytes())?;
    let _ = std::fs::remove_file(&compact_path);

    Ok((kept_count, compacted_count))
}

#[cfg(test)]
mod tests {
    use super::*;
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
}
