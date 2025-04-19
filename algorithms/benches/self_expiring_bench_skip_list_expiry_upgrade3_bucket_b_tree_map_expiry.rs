use algorithms::self_expiring::skip_list_with_expiry::upgrade3::bucket_b_tree_map_expiry::BucketBTreeMapExpiry;
use algorithms::self_expiring::skip_list_with_expiry::traits::ConcurrentTimeBasedExpiry;
use criterion::{black_box, criterion_group, criterion_main, BenchmarkId, Criterion};
use std::sync::Arc;
use std::time::{SystemTime, UNIX_EPOCH};

const TTL: usize = 300;

fn current_unix_time() -> usize {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs() as usize
}

fn bench_insert_upgrade3(c: &mut Criterion) {
    let mut group = c.benchmark_group("Upgrade3 BucketBTreeMapExpiry Insert");

    for &volume in &[10_000, 100_000, 1_000_000] {
        let now = current_unix_time();

        group.bench_with_input(
            BenchmarkId::new("Upgrade3 Insert", volume),
            &volume,
            |b, &volume| {
                b.iter(|| {
                    let expiry = Arc::new(BucketBTreeMapExpiry::new());
                    for i in 0..volume {
                        expiry.insert(format!("Item-{i}"), (now + (i % TTL)) as i64);
                    }
                });
            },
        );
    }

    group.finish();
}

fn bench_tick_upgrade3(c: &mut Criterion) {
    let mut group = c.benchmark_group("Upgrade3 BucketBTreeMapExpiry Ticking");
    let now = current_unix_time();
    let ticks = 1000;

    group.bench_function("Upgrade3 Tick 1000", |b| {
        let expiry = Arc::new(BucketBTreeMapExpiry::new());
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

criterion_group!(upgrade3_benches, bench_insert_upgrade3, bench_tick_upgrade3);
criterion_main!(upgrade3_benches);
