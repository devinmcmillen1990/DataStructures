use algorithms::self_expiring::split_list_with_expiry::base::split_list_with_expiry::SplitListExpiry;
use algorithms::self_expiring::split_list_with_expiry::traits::ConcurrentTimeBasedExpiry;
use criterion::{black_box, criterion_group, criterion_main, BenchmarkId, Criterion};
use std::time::{SystemTime, UNIX_EPOCH};

const TTL: usize = 300;

fn current_unix_time() -> usize {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs() as usize
}

fn bench_insert_split_list(c: &mut Criterion) {
    let mut group = c.benchmark_group("SplitListExpiry Insert");

    for &volume in &[10_000, 100_000, 1_000_000] {
        let now = current_unix_time();

        group.bench_with_input(BenchmarkId::new("Insert", volume), &volume, |b, &volume| {
            b.iter(|| {
                let expiry = SplitListExpiry::new();
                for i in 0..volume {
                    expiry.insert(format!("Item-{i}"), (now + (i % TTL)) as i64);
                }
            });
        });
    }

    group.finish();
}

fn bench_tick_split_list(c: &mut Criterion) {
    let mut group = c.benchmark_group("SplitListExpiry Ticking");

    let now = current_unix_time();
    let ticks = 1000;

    group.bench_function("Tick 1000", |b| {
        let expiry = SplitListExpiry::new();
        for i in 0..(ticks * 10) {
            expiry.insert(format!("Item-{i}"), (now + (i % TTL)) as i64);
        }

        b.iter(|| {
            for _ in 0..ticks {
                black_box(expiry.expire_front());
            }
        });
    });

    group.finish();
}

criterion_group!(
    split_list_benches,
    bench_insert_split_list,
    bench_tick_split_list
);
criterion_main!(split_list_benches);
