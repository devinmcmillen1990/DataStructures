use algorithms::self_expiring::skip_list_with_expiry::upgrade1::level_indexed_skip_list::LevelIndexedSkipList;
use algorithms::self_expiring::traits::IndexBasedExpiry;
use criterion::{black_box, criterion_group, criterion_main, BenchmarkId, Criterion};

const BUCKETS: usize = 64;

fn bench_insert_upgrade1(c: &mut Criterion) {
    let mut group = c.benchmark_group("Upgrade1 LevelIndexedSkipList");

    for &volume in &[10_000, 100_000, 1_000_000] {
        group.bench_with_input(
            BenchmarkId::new("Upgrade1 Insert", volume),
            &volume,
            |b, &volume| {
                b.iter(|| {
                    let mut skiplist = LevelIndexedSkipList::new(BUCKETS);
                    for i in 0..volume {
                        skiplist.insert(format!("Item-{i}"), i % BUCKETS);
                    }
                });
            },
        );
    }

    group.finish();
}

fn bench_tick_upgrade1(c: &mut Criterion) {
    let mut group = c.benchmark_group("Upgrade1 LevelIndexedSkipList Ticking");
    let ticks = 1000;

    group.bench_function("Upgrade1 Tick 1000", |b| {
        let mut skiplist = LevelIndexedSkipList::new(ticks);
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

criterion_group!(upgrade1_benches, bench_insert_upgrade1, bench_tick_upgrade1);
criterion_main!(upgrade1_benches);
