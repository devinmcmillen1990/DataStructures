# Time Indexed Algorithms
  ## Skip List with Expiry Buckets
    - cargo bench --bench time_indexed_bench_skip_list_expiry


# 📊 SkipListExpiry Variants: Performance Benchmark Report

**Date:** 2025-04-16 10:33:04

---

## 🔍 Overview

This benchmark analyzes and compares the performance of four time-indexed skip list implementations:

- `SkipListExpiry` (Original)
- `EnhancedSkipListExpiry1` (Improved insertion structure)
- `EnhancedSkipListExpiry2` (Bucket-based tracking with `IndexSet`)
- `EnhancedSkipListExpiry3` (Optimized `IndexMap` allocation + expiry handling)

Benchmarks were performed using Criterion.rs on volumes of 10K, 100K, and 1M insertions, including tick() performance and a real-world concurrent simulation.

---

## 📈 Fixed-Volume Insertions

### 🔢 Insert 10,000

| Implementation               | Time (µs)         |
|-----------------------------|-------------------|
| SkipListExpiry              | 1,334,000 µs      |
| EnhancedSkipListExpiry1     | 850,250 µs        |
| EnhancedSkipListExpiry2     | 830,780 µs        |
| EnhancedSkipListExpiry3     | 847,340 µs        |

### 🔢 Insert 100,000

| Implementation               | Time (ms)         |
|-----------------------------|-------------------|
| SkipListExpiry              | 15.230 ms         |
| EnhancedSkipListExpiry1     | 8.344 ms          |
| EnhancedSkipListExpiry2     | 8.152 ms          |
| EnhancedSkipListExpiry3     | 8.300 ms          |

### 🔢 Insert 1,000,000

| Implementation               | Time (ms)         |
|-----------------------------|-------------------|
| SkipListExpiry              | 260.280 ms        |
| EnhancedSkipListExpiry1     | 141.010 ms        |
| EnhancedSkipListExpiry2     | 139.240 ms        |
| EnhancedSkipListExpiry3     | 143.140 ms        |

---

## ⏳ Tick Expiry Performance (1000 Ticks)

| Implementation               | Time (µs)         |
|-----------------------------|-------------------|
| EnhancedSkipListExpiry2     | 69.45 µs          |
| EnhancedSkipListExpiry3     | 237.90 µs         |

> ✅ **Observation**: EnhancedSkipListExpiry2 is **over 3x faster** at tick expiry.

---

## 🌐 Real-World Simulation (Random Insertions + Concurrent Threads)

### 4 Threads × 2500 Insertions + 100 Expiry Ticks

| Implementation               | Time (ms)         |
|-----------------------------|-------------------|
| EnhancedSkipListExpiry3     | 1.59 ms           |

> 🧠 This tests concurrent access under realistic TTL patterns with multi-threaded inserts.

---

## ⚖️ Conclusions

- `EnhancedSkipListExpiry2` offers the **best tick performance** and **scales efficiently** across all insertion volumes.
- `EnhancedSkipListExpiry3` introduces `IndexMap` and memory optimizations, trading **slightly slower expiry** for **more balanced overall performance**.
- `SkipListExpiry` (original) struggles beyond 100K inserts and should be avoided for high-throughput systems.

---

## 📌 Recommendations

| Use Case                              | Best Variant                  |
|--------------------------------------|-------------------------------|
| High-volume insertions               | `EnhancedSkipListExpiry2`     |
| Real-time expiry tracking            | `EnhancedSkipListExpiry2`     |
| Balanced performance (general use)   | `EnhancedSkipListExpiry3`     |
| Single-threaded light use            | `EnhancedSkipListExpiry1`     |

---

## 🛠️ Notes

- Measurement time: **60 seconds**
- Sample size: **100 per test**
- Platform: Local machine (Windows, x86_64)
- Outlier handling: Criterion default