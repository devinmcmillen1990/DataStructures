# ⏱️ SkipListExpiry — Time-Bucketed Expiry Queue

**SkipListExpiry** is a simplified, performant data structure for managing time-based expirations, inspired by skip lists and timing wheels. It buckets items by time into discrete intervals and supports efficient `insert`, `expire`, and `len` operations.

This implementation is ideal for use cases like:
- Cache eviction
- Rate limiting
- Delayed job queues
- Prediction markets & expiry management

---

## 📚 Origin

This structure is inspired by skip lists and timing wheels, but does not implement layered probabilistic skip pointers. Instead, it uses a circular buffer of time buckets (VecDeque<BTreeSet<T>>) to simulate discrete-tick time progression for expiration. Based loosely on ideas from timing wheels and scheduling queues.
---

## 🚀 Design Summary

Items are mapped to time buckets using a resolution window (e.g., 1 second). A circular `VecDeque` of buckets (each a `BTreeSet`) holds all currently expiring items. When time advances, `expire_front()` rotates the queue, expiring all items in the oldest bucket.

### ⏱️ Core Properties

- **O(1)** insertion and expiration per tick
- Memory-efficient and easy to reason about
- No dependency on sorted skip list layers—buckets track items by time

---

## 🧪 Supported Operations

| Method              | Description                                              |
|---------------------|----------------------------------------------------------|
| `insert(id, time)`  | Adds `id` to the bucket corresponding to `time`.        |
| `expire_front()`    | Expires the oldest bucket and returns expired items.    |
| `len()`             | Returns number of active (non-expired) items.           |
| `is_empty()`        | Returns true if no items are currently tracked.         |

---

## 📋 Pseudocode

```pseudo
function insert(id, time):
    offset = (time - bucket_start_time) / resolution
    if offset < 0 or offset >= num_buckets:
        ignore
    else:
        buckets[offset].add(id)
        id_to_bucket[id] = offset

function expire_front():
    expired = buckets.pop_front()
    buckets.push_back(empty set)
    for id in expired:
        remove id from id_to_bucket
    return expired

function len():
    return size of id_to_bucket
```

---

## ✅ Test Coverage

### ✅ Core Functional Tests

| Test Name                             | Purpose                                                              |
|--------------------------------------|----------------------------------------------------------------------|
| `test_basic_insertion_and_expiry`    | Verifies items expire at the right time.                            |
| `test_expire_front_clears_bucket`    | Ensures buckets are recycled and cleared.                           |
| `test_ignore_out_of_range_items`     | Verifies that out-of-bounds insertions are ignored.                 |
| `test_len_and_is_empty_consistency`  | Confirms size tracking works with inserts and expirations.          |
| `test_multiple_items_same_bucket`    | Ensures grouping and batch expiration works per bucket.             |

### ✅ Edge Case Tests

| Test Name                              | Purpose                                                              |
|---------------------------------------|----------------------------------------------------------------------|
| `test_zero_items_expire_empty`        | Calling expiration on an empty structure should return `[]`.        |
| `test_insert_exactly_on_boundary`     | Inserting at exact bucket boundary is handled correctly.            |
| `test_duplicate_insert_overwrite`     | Overwriting existing keys places them in the correct bucket.        |
| `test_expire_all_buckets_and_reuse`   | Ensures circular buffer rotates and remains correct.                |

### 🚫 Concurrency Safety Tests

| Test Name                                     | Status           | Reason                                                 |
|----------------------------------------------|------------------|--------------------------------------------------------|
| `test_concurrent_insertions`                 | ❌ Not Applicable | Uses a global `Mutex`, no per-bucket locking.         |
| `test_concurrent_expiry_behavior`            | ❌ Not Applicable | Race conditions possible with `VecDeque`.             |
| `test_concurrent_insertions_stressed`        | ❌ Not Applicable | High-frequency concurrent insertions unsupported.     |
| `test_concurrent_expiry_behavior_stressed`   | ❌ Not Applicable | No support for stress-tested concurrent expirations.  |
| `test_concurrent_mixed_read_write_stressed`  | ❌ Not Applicable | Mixed workloads not synchronized with fine-grained locks. |
