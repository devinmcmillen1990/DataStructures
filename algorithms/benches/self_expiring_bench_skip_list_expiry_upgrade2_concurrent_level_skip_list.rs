use algorithms::self_expiring::skip_list_with_expiry::upgrade2::concurrent_level_skip_list::ConcurrentLevelSkipList;
use algorithms::self_expiring::skip_list_with_expiry::traits::ConcurrentIndexBasedExpiry;
use criterion::{black_box, criterion_group, criterion_main, BenchmarkId, Criterion};
use std::sync::Arc;

const BUCKETS: usize = 64;

fn bench_insert_upgrade2(c: &mut Criterion) {
    let mut group = c.benchmark_group("Upgrade2 SkipListConcurrentLevel");

    for &volume in &[10_000, 100_000, 1_000_000] {
        group.bench_with_input(
            BenchmarkId::new("Upgrade2 Insert", volume),
            &volume,
            |b, &volume| {
                b.iter(|| {
                    let skiplist = Arc::new(ConcurrentLevelSkipList::new(BUCKETS));
                    for i in 0..volume {
                        skiplist.insert(format!("Item-{i}"), i % BUCKETS);
                    }
                });
            },
        );
    }

    group.finish();
}

fn bench_tick_upgrade2(c: &mut Criterion) {
    let mut group = c.benchmark_group("Upgrade2 SkipListConcurrentLevel Ticking");
    let ticks = 1000;

    group.bench_function("Upgrade2 Tick 1000", |b| {
        let skiplist = Arc::new(ConcurrentLevelSkipList::new(ticks));
        for i in 0..(ticks * 10) {
            skiplist.insert(format!("Item-{i}"), i % ticks);
        }

        b.iter(|| {
            for _ in 0..ticks {
                black_box(skiplist.expire_front());
            }
        });
    });

    group.finish();
}

criterion_group!(upgrade2_benches, bench_insert_upgrade2, bench_tick_upgrade2);
criterion_main!(upgrade2_benches);
