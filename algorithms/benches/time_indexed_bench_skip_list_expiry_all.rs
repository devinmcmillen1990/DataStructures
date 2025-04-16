// benches/time_indexed_bench_all.rs

use algorithms::time_indexed::skip_list_expiry::SkipListExpiry;
use algorithms::time_indexed::time_segmented_heap::TimeSegmentedHeap;
use algorithms::time_indexed::timing_wheel::TimingWheel;
use algorithms::time_indexed_enhanced::skip_list_expiry_enhanced::EnhancedSkipListExpiry;
use algorithms::time_indexed_enhanced2::skip_list_expiry_enhanced2::EnhancedSkipListExpiry2;
use criterion::{criterion_group, criterion_main, Criterion};
use std::time::{SystemTime, UNIX_EPOCH};

fn bench_all_skiplists(c: &mut Criterion) {
    let mut group = c.benchmark_group("SkipListExpiry Variants");
    let now = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs() as usize;

    group.bench_function("SkipListExpiry/Insert 10k", |b| {
        b.iter(|| {
            let skiplist = SkipListExpiry::new(300, 1);
            for i in 0..10_000 {
                skiplist.insert(format!("Item-{i}"), (now + i % 300) as i64);
            }
        });
    });

    group.bench_function("EnhancedSkipListExpiry/Insert 10k", |b| {
        b.iter(|| {
            let skiplist = EnhancedSkipListExpiry::with_start_time(300, 1, now);
            for i in 0..10_000 {
                skiplist.insert(format!("Item-{i}"), now + i % 300);
            }
        });
    });

    group.bench_function("EnhancedSkipListExpiry2/Insert 10k", |b| {
        b.iter(|| {
            let skiplist = EnhancedSkipListExpiry2::with_start_time(300, 1, now);
            for i in 0..10_000 {
                skiplist.insert(format!("Item-{i}"), now + i % 300);
            }
        });
    });

    group.finish();
}

criterion_group!(benches, bench_all_skiplists);
criterion_main!(benches);
