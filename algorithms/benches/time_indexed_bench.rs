use criterion::{black_box, criterion_group, criterion_main, Criterion};
use algorithms::time_indexed::{
    skip_list_expiry::SkipListExpiry,
    time_segmented_heap::TimeSegmentedHeap,
    timing_wheel::TimingWheel,
};
use chrono::Utc;
use rand::Rng;

fn enhanced_benchmarks(c: &mut Criterion) {
    let now = Utc::now().timestamp();

    // Insert with extended TTL
    c.bench_function("SkipListExpiry - Insert 10,000 with TTL 300s", |b| {
        b.iter(|| {
            let skiplist: SkipListExpiry<String> = SkipListExpiry::new(100, 10);
            for i in 0..10_000 {
                skiplist.insert(format!("item-{i}"), black_box(now + 300));
            }
        })
    });

    c.bench_function("TimeSegmentedHeap - Insert 10,000 with TTL 300s", |b| {
        b.iter(|| {
            let heap: TimeSegmentedHeap<String> = TimeSegmentedHeap::new(100, 10);
            for i in 0..10_000 {
                heap.insert(format!("item-{i}"), black_box(now + 300));
            }
        })
    });

    c.bench_function("TimingWheel - Insert 10,000 with TTL 300s", |b| {
        b.iter(|| {
            let mut wheel: TimingWheel<String> = TimingWheel::new(100, 10);
            for i in 0..10_000 {
                wheel.insert(format!("item-{i}"), black_box(now + 300));
            }
        })
    });

    // High-frequency expiry
    c.bench_function("SkipListExpiry - Tick 1000 times", |b| {
        let skiplist: SkipListExpiry<String> = SkipListExpiry::new(1000, 1);
        b.iter(|| {
            for _ in 0..1000 {
                black_box(skiplist.expire_front());
            }
        })
    });

    c.bench_function("TimeSegmentedHeap - Tick 1000 times", |b| {
        let heap: TimeSegmentedHeap<String> = TimeSegmentedHeap::new(1000, 1);
        b.iter(|| {
            for _ in 0..1000 {
                black_box(heap.expire_front());
            }
        })
    });

    c.bench_function("TimingWheel - Tick 1000 times", |b| {
        let mut wheel: TimingWheel<String> = TimingWheel::new(1000, 1);
        b.iter(|| {
            for _ in 0..1000 {
                black_box(wheel.tick());
            }
        })
    });

    // Random/skewed TTL insert
    c.bench_function("TimingWheel - Randomized TTL Inserts", |b| {
        let mut rng = rand::thread_rng();
        b.iter(|| {
            let mut wheel: TimingWheel<String> = TimingWheel::new(100, 10);
            for i in 0..10_000 {
                let skewed_ttl = rng.gen_range(10..=600);
                wheel.insert(format!("item-{i}"), now + skewed_ttl);
            }
        })
    });

    // Memory estimate print
    let mut wheel: TimingWheel<String> = TimingWheel::new(100, 10);
    for i in 0..10_000 {
        wheel.insert(format!("item-{i}"), now + 300);
    }
    println!("TimingWheel Memory Estimate (len): {}", wheel.len());
}

criterion_group!(benches, enhanced_benchmarks);
criterion_main!(benches);
