# SkipListExpiry Benchmark Analysis

**Date:** 2025-04-16

---

## 📊 Benchmark Summary

We evaluated 3 variants of `SkipListExpiry` across multiple benchmark suites:

### `time_indexed_bench_all.rs`

| Variant                  | Insert 10k (Mean) |
|--------------------------|------------------|
| SkipListExpiry           | ~3.30 ms         |
| EnhancedSkipListExpiry   | ~3.10 ms         |
| EnhancedSkipListExpiry2  | ~2.68 ms ✅       |

✅ **Enhanced2** showed the fastest insertion time, improving over:
- Original: ~19%
- Enhanced1: ~13.7%

---

### `time_indexed_bench_skip_list_expiry_enhanced.rs`

| Variant                  | Insert 10k (Mean) | Tick 1000 (Mean) |
|--------------------------|------------------|------------------|
| SkipListExpiry           | ~3.58 ms         | ~34.6 µs ✅        |
| EnhancedSkipListExpiry   | ~862 µs ✅        | ~305 µs ❌        |

- **Insert Speed**: Enhanced1 dominated insertions.
- **Tick Speed**: SkipListExpiry performed better, likely due to lightweight operations on expiration buckets.

---

### `time_indexed_bench_skip_list_expiry_enhanced2.rs`

| Variant                  | Insert 10k (Mean) | Performance vs Baseline |
|--------------------------|------------------|--------------------------|
| SkipListExpiry           | ~3.24 ms         | baseline                 |
| EnhancedSkipListExpiry   | ~2.92 ms         | ~6% faster               |
| EnhancedSkipListExpiry2  | ~2.60 ms ✅        | ~20% faster              |

✅ **Enhanced2** is the top performer for insertion-heavy workloads.

---

## ⚙️ Running Individual Benches

To run a single benchmark file:

```bash
cargo bench --bench <file_name>
```

Examples:

```bash
cargo bench --bench time_indexed_bench_skip_list_expiry_enhanced2
```

To run a specific group inside a benchmark:

```bash
cargo bench --bench time_indexed_bench_skip_list_expiry_enhanced2 -- EnhancedSkipListExpiry2
```

Or a specific function path:

```bash
cargo bench --bench time_indexed_bench_skip_list_expiry_enhanced2 -- "SkipListExpiry Variants/EnhancedSkipListExpiry2/Insert 10k"
```

Use `-- --nocapture` to display `println!()` output.

---

## 📌 Next Steps

- Optimize tick logic in `Enhanced2`
- Investigate memory re-use and low-level heap pressure
- Compare multi-threaded inserts
- Explore lock-free skip list structures
