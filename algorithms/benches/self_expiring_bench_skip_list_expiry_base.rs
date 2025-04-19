use algorithms::self_expiring::skip_list_with_expiry::base::skip_list_expiry::SkipListExpiry;
use algorithms::self_expiring::skip_list_with_expiry::traits::TimeBasedExpiry;
use criterion::{black_box, criterion_group, criterion_main, BenchmarkId, Criterion};
use std::time::{SystemTime, UNIX_EPOCH};

const TTL: usize = 300;

fn current_unix_time() -> usize {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs() as usize
}

fn bench_insert_base(c: &mut Criterion) {
    let mut group = c.benchmark_group("Base SkipListExpiry");

    for &volume in &[10_000, 100_000, 1_000_000] {
        let now = current_unix_time();

        group.bench_with_input(
            BenchmarkId::new("Base Insert", volume),
            &volume,
            |b, &volume| {
                b.iter(|| {
                    let mut skiplist = SkipListExpiry::new(64, 1);
                    for i in 0..volume {
                        skiplist.insert(format!("Item-{i}"), (now + (i % TTL)) as i64);
                    }
                });
            },
        );
    }

    group.finish();
}

fn bench_tick_base(c: &mut Criterion) {
    let mut group = c.benchmark_group("Base SkipListExpiry Ticking");
    let now = current_unix_time();
    let ticks = 1000;

    group.bench_function("Base Tick 1000", |b| {
        let mut skiplist = SkipListExpiry::new(ticks, 1);
        for i in 0..(ticks * 10) {
            skiplist.insert(format!("Item-{i}"), (now + (i % TTL)) as i64);
        }

        b.iter(|| {
            for _ in 0..ticks {
                black_box(skiplist.expire_front());
            }
        });
    });

    group.finish();
}

criterion_group!(base_benches, bench_insert_base, bench_tick_base);
criterion_main!(base_benches);
