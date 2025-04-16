
use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion};
use algorithms::time_indexed::skip_list_expiry::SkipListExpiry;
use algorithms::time_indexed_enhanced::skip_list_expiry_enhanced::EnhancedSkipListExpiry;
use chrono::Utc;

fn bench_skip_lists(c: &mut Criterion) {
    let mut group = c.benchmark_group("SkipList vs EnhancedSkipList");

    group.bench_function(BenchmarkId::new("SkipListExpiry", "Insert 10000"), |b| {
        b.iter(|| {
            let skiplist = SkipListExpiry::new(100, 1);
            let now = Utc::now().timestamp();
            for i in 0..10_000 {
                skiplist.insert(format!("Item-{i}"), now + ((i % 100) as i64));
            }
        });
    });

    group.bench_function(BenchmarkId::new("EnhancedSkipListExpiry", "Insert 10000"), |b| {
        b.iter(|| {
            let skiplist = EnhancedSkipListExpiry::new(100, 1);
            let now = Utc::now().timestamp() as usize;
            for i in 0..10_000 {
                skiplist.insert(format!("Item-{i}"), now + ((i % 100) as usize));
            }
        });
    });

    group.bench_function(BenchmarkId::new("SkipListExpiry", "Tick 1000"), |b| {
        let skiplist = SkipListExpiry::new(1000, 1);
        let now = Utc::now().timestamp();
        for i in 0..10_000 {
            skiplist.insert(format!("Item-{i}"), now + ((i % 1000) as i64));
        }
        b.iter(|| {
            for _ in 0..1000 {
                skiplist.expire_front();
            }
        });
    });

    group.bench_function(BenchmarkId::new("EnhancedSkipListExpiry", "Tick 1000"), |b| {
        let skiplist = EnhancedSkipListExpiry::new(1000, 1);
        let now = Utc::now().timestamp() as usize;
        for i in 0..10_000 {
            skiplist.insert(format!("Item-{i}"), now + ((i % 1000) as usize));
        }
        b.iter(|| {
            for _ in 0..1000 {
                skiplist.tick();
            }
        });
    });

    group.finish();
}

criterion_group!(benches, bench_skip_lists);
criterion_main!(benches);
