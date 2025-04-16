# 📊 SkipList vs EnhancedSkipList — Initial Benchmark Assessment

## 🧪 Benchmark Overview

This benchmark compares the performance of two self-expiring data structures:

- `SkipListExpiry`: Original implementation using `BTreeSet` buckets.
- `EnhancedSkipListExpiry`: Optimized version using `Vec<T>` + `HashMap` for deduplication and simplified memory layout.

Each structure is tested on two key operations:

- `insert()` — inserting elements with a TTL.
- `tick()` — expiring the oldest bucket of entries.

---

## ⚙️ Benchmark Results

| Operation                   | SkipListExpiry     | EnhancedSkipListExpiry | Speedup / Slowdown |
|----------------------------|--------------------|-------------------------|--------------------|
| **Insert 10,000 Elements** | `~3.54 ms`         | `~0.84 ms`              | 🚀 ~**4x faster**   |
| **Tick 1000 Times**        | `~33.7 µs`         | `~303 µs`               | 🐢 ~**9x slower**   |

---

## ✅ Insert Performance

The enhanced skip list outperformed the original significantly:

- **~4x faster** inserts at scale.
- Improved due to:
  - Flat memory layout (`Vec<T>` vs `BTreeSet`).
  - No per-element ordering cost.
  - Constant-time deduplication with `HashMap`.

> 💡 Ideal for bursty workloads with frequent inserts and infrequent expirations (e.g. snapshot-based TTLs).

---

## ⚠️ Tick Performance

The original skip list performed better on `tick()` operations:

- Enhanced version is ~**9x slower** at `tick()`.
- Slower due to:
  - `Vec::drain()` and `rotate_left()` being less cache-friendly.
  - Rebuilding bucket vectors every tick incurs allocation costs.

> ⚠️ Enhanced version trades off ticking performance for insertion throughput.

---

## 🧠 Summary

| Scenario                          | Recommended Implementation       |
|----------------------------------|----------------------------------|
| High-throughput inserts, low tick frequency | ✅ `EnhancedSkipListExpiry` |
| High-frequency expiration (real-time)       | ✅ `SkipListExpiry`          |
| Balanced use-case                         | Consider hybrid or configurable strategy |

---

## 🔭 Next Steps

- Explore replacing `Vec<T>` with `VecDeque<T>` to improve `tick()` cost.
- Benchmark memory footprint and latency at higher throughput (1M+ entries).
- Consider using fixed-capacity slabs or reusable buffers.
- Proceed to benchmarking and optimizing the `TimingWheel` and `TimeSegmentedHeap`.
