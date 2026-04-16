use criterion::{black_box, criterion_group, criterion_main, BatchSize, Criterion};
use std::collections::BTreeMap;
use std::fs::{create_dir_all, read_to_string, write};
use std::io::Write;
use std::path::PathBuf;
use std::time::{Duration, Instant};

use tempfile::tempdir;
use xbreed::mailbox::{compact_events, drain_events, write_event, Event};

const ITERATIONS: usize = 25;
const SIZES: [usize; 4] = [1, 100, 10_000, 100_000];

fn mailbox_path(team_dir: &std::path::Path) -> PathBuf {
    team_dir
        .join(".xbreed")
        .join("mailbox")
        .join("events.ndjson")
}

fn seed_events(team_dir: &std::path::Path, count: usize) {
    for i in 0..count {
        write_event(team_dir, &format!("seed-{i}"), "seeded", "payload").unwrap();
    }
}

fn seed_old_events(team_dir: &std::path::Path, count: usize) {
    let path = mailbox_path(team_dir);
    create_dir_all(path.parent().unwrap()).unwrap();
    let mut file = std::fs::OpenOptions::new()
        .create(true)
        .append(true)
        .open(path)
        .unwrap();
    for i in 0..count {
        let event = Event {
            timestamp_ms: 1,
            from: format!("seed-old-{i}"),
            event_type: "seeded".to_string(),
            payload: "payload".to_string(),
        };
        let mut line = serde_json::to_string(&event).unwrap();
        line.push('\n');
        file.write_all(line.as_bytes()).unwrap();
    }
}

fn percentile_ms(samples: &mut [f64], p: f64) -> f64 {
    samples.sort_by(|a, b| a.total_cmp(b));
    let len = samples.len();
    let idx = ((len as f64 - 1.0) * p)
        .round()
        .clamp(0.0, (len - 1) as f64) as usize;
    samples[idx]
}

fn to_ms(duration: Duration) -> f64 {
    duration.as_secs_f64() * 1_000.0
}

fn collect_write_event_ms(count: usize) -> (f64, f64) {
    let mut samples = Vec::with_capacity(ITERATIONS);
    for _ in 0..ITERATIONS {
        let dir = tempdir().unwrap();
        seed_events(dir.path(), count);
        let start = Instant::now();
        write_event(dir.path(), "writer", "bench", "payload").unwrap();
        samples.push(to_ms(start.elapsed()));
    }
    let p50 = percentile_ms(&mut samples, 0.50);
    let p95 = percentile_ms(&mut samples, 0.95);
    (p50, p95)
}

fn collect_drain_events_ms(count: usize) -> (f64, f64) {
    let mut samples = Vec::with_capacity(ITERATIONS);
    for _ in 0..ITERATIONS {
        let dir = tempdir().unwrap();
        seed_events(dir.path(), count);
        let start = Instant::now();
        black_box(drain_events(dir.path()).unwrap());
        samples.push(to_ms(start.elapsed()));
    }
    let p50 = percentile_ms(&mut samples, 0.50);
    let p95 = percentile_ms(&mut samples, 0.95);
    (p50, p95)
}

fn collect_compact_events_ms(count: usize) -> (f64, f64) {
    let mut samples = Vec::with_capacity(ITERATIONS);
    for _ in 0..ITERATIONS {
        let dir = tempdir().unwrap();
        seed_old_events(dir.path(), count);
        let keep_types: Vec<String> = vec![];
        let start = Instant::now();
        black_box(compact_events(dir.path(), &keep_types, 0).unwrap());
        samples.push(to_ms(start.elapsed()));
    }
    let p50 = percentile_ms(&mut samples, 0.50);
    let p95 = percentile_ms(&mut samples, 0.95);
    (p50, p95)
}

fn emit_functional_baseline() {
    let mut payload = BTreeMap::<String, BTreeMap<String, serde_json::Value>>::new();
    let mut write_events = BTreeMap::new();
    let mut drain_events_stats = BTreeMap::new();
    let mut compact_events_stats = BTreeMap::new();

    for size in SIZES {
        let (w_p50, w_p95) = collect_write_event_ms(size);
        write_events.insert(
            format!("n={size}"),
            serde_json::json!({"p50_ms": w_p50, "p95_ms": w_p95}),
        );

        let (d_p50, d_p95) = collect_drain_events_ms(size);
        drain_events_stats.insert(
            format!("n={size}"),
            serde_json::json!({"p50_ms": d_p50, "p95_ms": d_p95}),
        );

        let (c_p50, c_p95) = collect_compact_events_ms(size);
        compact_events_stats.insert(
            format!("n={size}"),
            serde_json::json!({"p50_ms": c_p50, "p95_ms": c_p95}),
        );
    }

    payload.insert("write_event".to_string(), write_events);
    payload.insert("drain_events".to_string(), drain_events_stats);
    payload.insert("compact_events".to_string(), compact_events_stats);

    let manifest_dir = env!("CARGO_MANIFEST_DIR");
    let report_path = PathBuf::from(manifest_dir).join("docs/reports/mailbox-bench-baseline.json");
    if let Some(parent) = report_path.parent() {
        create_dir_all(parent).unwrap();
    }
    let mut report: serde_json::Value = read_to_string(&report_path)
        .ok()
        .and_then(|content| serde_json::from_str(&content).ok())
        .unwrap_or_else(|| serde_json::json!({}));
    report["function_wall_ms"] = serde_json::json!(payload);
    write(&report_path, serde_json::to_string_pretty(&report).unwrap()).unwrap();
}

fn bench_mailbox(c: &mut Criterion) {
    emit_functional_baseline();

    for size in SIZES {
        let bid = format!("write_event/{size}");
        c.bench_function(bid.as_str(), |b| {
            b.iter_batched(
                || {
                    let dir = tempdir().unwrap();
                    seed_events(dir.path(), size);
                    dir
                },
                |dir| {
                    write_event(
                        dir.path(),
                        black_box("writer"),
                        black_box("bench"),
                        black_box("payload"),
                    )
                    .unwrap();
                },
                BatchSize::PerIteration,
            )
        });

        let bid = format!("drain_events/{size}");
        c.bench_function(bid.as_str(), |b| {
            b.iter_batched(
                || {
                    let dir = tempdir().unwrap();
                    seed_events(dir.path(), size);
                    dir
                },
                |dir| {
                    black_box(drain_events(dir.path()).unwrap());
                },
                BatchSize::PerIteration,
            )
        });

        let bid = format!("compact_events/{size}");
        c.bench_function(bid.as_str(), |b| {
            b.iter_batched(
                || {
                    let dir = tempdir().unwrap();
                    seed_old_events(dir.path(), size);
                    dir
                },
                |dir| {
                    let keep_types: Vec<String> = vec![];
                    black_box(compact_events(dir.path(), &keep_types, 0).unwrap());
                },
                BatchSize::PerIteration,
            )
        });
    }
}

criterion_group!(mailbox_benches, bench_mailbox);
criterion_main!(mailbox_benches);
