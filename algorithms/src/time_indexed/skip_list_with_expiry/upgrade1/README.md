# 🧱 LevelIndexedSkipList — Hierarchical Expiry Buckets

**LevelIndexedSkipList** is an upgrade from `SkipListExpiry`, designed to bucket items by *abstract levels* rather than absolute time. This structure is ideal when expiration events follow a priority or tiered system, rather than strict time slots.

Use cases include:
- Multi-tiered task queues (e.g., Level 0 = immediate, Level 3 = low priority)
- Quota or tiered billing management
- Tiered prediction market closures or event staging

---

## 🧠 Design Summary

The structure uses a vector of `BTreeSet<T>` buckets, indexed by level:

```text
+-------+-------+-------+-------+
| L=0   | L=1   | L=2   | L=3   |
|-------|-------|-------|-------|
| {A}   | {}    | {B}   | {}    |
+-------+-------+-------+-------+
```

Each item is assigned to a level explicitly. Internally, `id_to_level` tracks the current level for each `T`, ensuring overwrite semantics and fast expiration.

### ⚙️ Expiry Model

- Items are inserted into a specific level (`usize`)
- `expire_front()` removes and returns the *first non-empty* level's items
- Optional support for promotion (not enabled by default)

---

## 🚀 Core Properties

- **O(1)** lookup and insertion (amortized)
- Level-aware, priority-expiring semantics
- Simple internal model: `Vec<BTreeSet<T>> + HashMap<T, usize>`

---

## 🧪 Supported Operations

| Method              | Description                                                        |
|---------------------|--------------------------------------------------------------------|
| `insert(id, level)` | Adds `id` to the given level, overwriting if it already exists.   |
| `expire_front()`    | Removes and returns items in the earliest non-empty level bucket. |
| `len()`             | Returns number of tracked items.                                  |
| `is_empty()`        | Returns true if no items are present.                             |
| `values()`          | Snapshot of all tracked items (unordered).                        |

---

## ✅ Test Coverage

### ✅ Core Functional Tests

| Test Name                             | Purpose                                                           |
|--------------------------------------|-------------------------------------------------------------------|
| `test_basic_insertion_and_expiry`    | Verifies items expire in lowest-level-first order.               |
| `test_expire_front_clears_bucket`    | Confirms level bucket is cleared properly.                       |
| `test_ignore_out_of_range_items`     | Verifies bounds-check for levels is respected.                   |
| `test_len_and_is_empty_consistency`  | Confirms size tracking and emptiness are consistent.             |
| `test_multiple_items_same_bucket`    | Confirms group expiry at same level.                             |
| `test_values_snapshot_consistency`   | Validates snapshot logic against inserts.                        |

### ✅ Edge Case Tests

| Test Name                              | Purpose                                                              |
|---------------------------------------|----------------------------------------------------------------------|
| `test_zero_items_expire_empty`        | Expiring from empty list returns empty vector.                       |
| `test_expire_all_buckets_and_reuse`   | Verifies repeated expirations rotate through buckets cleanly.        |
| `test_len_decreases_after_expiry`     | Checks consistency as each bucket is expired.                        |
| `test_insert_exactly_on_boundary`     | Not applicable — no time boundaries in level-based structure.        |
| `test_duplicate_insert_overwrite`     | Confirms overwrite semantics when inserting same ID multiple times.  |
| `test_insert_and_expire_multiple_cycles` | Inserts into levels, expires repeatedly to test durability.       |

### ❌ Concurrency Safety Tests

| Test Name                                     | Status           | Reason                                                   |
|----------------------------------------------|------------------|----------------------------------------------------------|
| `test_concurrent_insertions`                 | ❌ Not Applicable | Not thread-safe (no internal sync).                     |
| `test_concurrent_expiry_behavior`            | ❌ Not Applicable | Concurrent access not supported.                        |
| `test_concurrent_insertions_stressed`        | ❌ Not Applicable | No per-level locking — race conditions would emerge.    |
| `test_concurrent_expiry_behavior_stressed`   | ❌ Not Applicable | Stress concurrency not safe in current model.           |
| `test_concurrent_mixed_read_write_stressed`  | ❌ Not Applicable | Snapshot + expiry race could corrupt state.             |