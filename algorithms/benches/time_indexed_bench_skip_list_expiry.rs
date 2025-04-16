use algorithms::time_indexed::skip_list_expiry::SkipListExpiry;
use algorithms::time_indexed_enhanced1::skip_list_expiry_enhanced1::EnhancedSkipListExpiry1;
use algorithms::time_indexed_enhanced2::skip_list_expiry_enhanced2::EnhancedSkipListExpiry2;
use algorithms::time_indexed_enhanced3::skip_list_expiry_enhanced3::EnhancedSkipListExpiry3;
use algorithms::time_indexed_enhanced4::skip_list_expiry_enhanced4::EnhancedSkipListExpiry4;
use algorithms::time_indexed_enhanced5::skip_list_expiry_enhanced5::EnhancedSkipListExpiry5;
use criterion::{black_box, criterion_group, criterion_main, BenchmarkId, Criterion};
use rand::{thread_rng, Rng};
use std::sync::Arc;
use std::thread;
use std::time::{SystemTime, UNIX_EPOCH};

const TTL: usize = 300;

fn current_unix_time() -> usize {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs() as usize
}

fn bench_insert(c: &mut Criterion) {
    let mut group = c.benchmark_group("SkipListExpiry Variants");

    for &volume in &[10_000, 100_000, 1_000_000] {
        let now = current_unix_time();

        group.bench_with_input(BenchmarkId::new("SkipListExpiry", volume), &volume, |b, &volume| {
            b.iter(|| {
                let skiplist = SkipListExpiry::new(64, 1);
                for i in 0..volume {
                    skiplist.insert(format!("Item-{i}"), (now + (i % TTL)) as i64);
                }
            });
        });

        group.bench_with_input(
            BenchmarkId::new("EnhancedSkipListExpiry1", volume),
            &volume,
            |b, &volume| {
                b.iter(|| {
                    let skiplist = EnhancedSkipListExpiry1::new(64, 1);
                    for i in 0..volume {
                        skiplist.insert(format!("Item-{i}"), now + (i % TTL));
                    }
                });
            },
        );

        group.bench_with_input(
            BenchmarkId::new("EnhancedSkipListExpiry2", volume),
            &volume,
            |b, &volume| {
                b.iter(|| {
                    let skiplist = EnhancedSkipListExpiry2::new(64, 1);
                    for i in 0..volume {
                        skiplist.insert(format!("Item-{i}"), now + (i % TTL));
                    }
                });
            },
        );

        group.bench_with_input(
            BenchmarkId::new("EnhancedSkipListExpiry3", volume),
            &volume,
            |b, &volume| {
                b.iter(|| {
                    let skiplist = EnhancedSkipListExpiry3::new(64, 1);
                    for i in 0..volume {
                        skiplist.insert(format!("Item-{i}"), now + (i % TTL));
                    }
                });
            },
        );

        group.bench_with_input(
            BenchmarkId::new("EnhancedSkipListExpiry4", volume),
            &volume,
            |b, &volume| {
                b.iter(|| {
                    let skiplist = EnhancedSkipListExpiry4::new();
                    for i in 0..volume {
                        skiplist.insert(format!("Item-{i}"), now + (i % TTL));
                    }
                });
            },
        );

        group.bench_with_input(
            BenchmarkId::new("EnhancedSkipListExpiry5", volume),
            &volume,
            |b, &volume| {
                b.iter(|| {
                    let mut skiplist = EnhancedSkipListExpiry5::new(64);
                    for i in 0..volume {
                        skiplist.insert(format!("Item-{i}"), now + (i % TTL));
                    }
                });
            },
        );
    }

    group.finish();
}

fn bench_tick(c: &mut Criterion) {
    let mut group = c.benchmark_group("Tick Expiry Processing");
    let now = current_unix_time();
    let ticks = 1000;

    group.bench_function("EnhancedSkipListExpiry2 Tick 1000", |b| {
        let skiplist = EnhancedSkipListExpiry2::new(ticks, 1);
        for i in 0..(ticks * 10) {
            skiplist.insert(format!("Item-{i}"), now + (i % TTL));
        }
        b.iter(|| {
            for _ in 0..ticks {
                black_box(skiplist.tick());
            }
        });
    });

    group.bench_function("EnhancedSkipListExpiry3 Tick 1000", |b| {
        let skiplist = EnhancedSkipListExpiry3::new(ticks, 1);
        for i in 0..(ticks * 10) {
            skiplist.insert(format!("Item-{i}"), now + (i % TTL));
        }
        b.iter(|| {
            for _ in 0..ticks {
                black_box(skiplist.tick());
            }
        });
    });

    group.bench_function("EnhancedSkipListExpiry4 Tick 1000", |b| {
        let skiplist = EnhancedSkipListExpiry4::new();
        for i in 0..(ticks * 10) {
            skiplist.insert(format!("Item-{i}"), now + (i % TTL));
        }
        b.iter(|| {
            for t in 0..ticks {
                black_box(skiplist.tick(now + t));
            }
        });
    });

    group.bench_function("EnhancedSkipListExpiry5 Tick 1000", |b| {
        let mut skiplist = EnhancedSkipListExpiry5::new(ticks);
        for i in 0..(ticks * 10) {
            skiplist.insert(format!("Item-{i}"), now + (i % TTL));
        }
        b.iter(|| {
            for t in 0..ticks {
                black_box(skiplist.tick(now + t));
            }
        });
    });

    group.finish();
}

fn bench_real_world_sim(c: &mut Criterion) {
    let mut group = c.benchmark_group("RealWorld Random+Concurrent");

    group.bench_function("EnhancedSkipListExpiry3 Real World Sim", |b| {
        b.iter(|| {
            let skiplist = Arc::new(EnhancedSkipListExpiry3::new(300, 1));
            let mut handles = vec![];

            for _ in 0..4 {
                let s = Arc::clone(&skiplist);
                handles.push(thread::spawn(move || {
                    let mut rng = thread_rng();
                    for i in 0..2500 {
                        let ttl = rng.gen_range(1..300);
                        s.insert(format!("Item-{}", i), current_unix_time() + ttl);
                    }
                }));
            }

            for handle in handles {
                handle.join().unwrap();
            }

            for _ in 0..100 {
                black_box(skiplist.tick());
            }
        });
    });

    group.bench_function("EnhancedSkipListExpiry4 Real World Sim", |b| {
        b.iter(|| {
            let skiplist = Arc::new(EnhancedSkipListExpiry4::new());
            let mut handles = vec![];

            for _ in 0..4 {
                let s = Arc::clone(&skiplist);
                handles.push(thread::spawn(move || {
                    let mut rng = thread_rng();
                    for i in 0..2500 {
                        let ttl = rng.gen_range(1..300);
                        s.insert(format!("Item-{}", i), current_unix_time() + ttl);
                    }
                }));
            }

            for handle in handles {
                handle.join().unwrap();
            }

            for t in 0..100 {
                black_box(skiplist.tick(current_unix_time() + t));
            }
        });
    });

    group.bench_function("EnhancedSkipListExpiry5 Real World Sim", |b| {
        b.iter(|| {
            let skiplist = Arc::new(RwLock::new(EnhancedSkipListExpiry5::new(300)));
            let mut handles = vec![];

            for _ in 0..4 {
                let s = Arc::clone(&skiplist);
                handles.push(thread::spawn(move || {
                    let mut rng = thread_rng();
                    for i in 0..2500 {
                        let ttl = rng.gen_range(1..300);
                        s.write().unwrap().insert(format!("Item-{}", i), current_unix_time() + ttl);
                    }
                }));
            }

            for handle in handles {
                handle.join().unwrap();
            }

            for t in 0..100 {
                black_box(skiplist.write().unwrap().tick(current_unix_time() + t));
            }
        });
    });

    group.finish();
}

fn custom_criterion() -> Criterion {
    Criterion::default()
        .sample_size(100)
        .measurement_time(std::time::Duration::from_secs(60))
}

criterion_group! {
    name = skiplist_comprehensive;
    config = custom_criterion();
    targets = bench_insert, bench_tick, bench_real_world_sim
}

criterion_main!(skiplist_comprehensive);
