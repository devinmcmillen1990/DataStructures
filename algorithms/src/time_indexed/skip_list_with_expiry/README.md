# Skip List with Expiry Algorithm Alterations

| **Upgrade** | **Struct Name**               | **Based on Skip List?** | **Thread & Race Safe?** | **Motivation for Enhancement**                                            | **Notes**                                                                 |
|-------------|-------------------------------|--------------------------|--------------------------|---------------------------------------------------------------------------|---------------------------------------------------------------------------|
| base        | `SkipListExpiry`              | ✅                       | 🚫                       | Core structure using skip list for expiring elements                     | Simple and performant for single-threaded use                             |
| upgrade1    | `SkipListLevelIndexed`        | ✅                       | 🚫                       | Improve time precision via level-indexed bucketing                       | Level granularity enabled tighter expiry alignment                       |
| upgrade2    | `SkipListConcurrentLevel`     | ✅                       | ✅                       | Make level-indexed skip list thread-safe                                 | Added locking mechanisms to enable concurrent inserts and ticks          |
| upgrade3    | `SkipListFlatWheel`           | ✅                       | 🚫                       | Reduce memory & simplify design                                          | Flattened skip list concept for compactness, but sacrificed thread safety|
| upgrade4    | `BucketHashSetExpiry`         | ❌                       | 🚫                       | Improve insertion speed and simplicity                                   | Dropped skip list entirely for unordered bucket-hashset model            |
| upgrade5    | `BucketBTreeMapExpiry`        | ❌                       | ✅                       | Add expiry order tracking + thread safety                                | BTreeMap ensured order; HashMap for reverse lookup; RwLock for safety     |
| upgrade6    | `SplitListLinkedBuckets`      | ❌                       | ✅                       | Explore sequential data layout for expiry                                | Buckets internally organized as linked lists with expiry groups          |
| upgrade7    | `SplitListTrieIndexed`        | ❌                       | 🚫                       | Investigate indexed lookup scalability                                   | Trie-based indexing for large-scale lookups, but lacked sync             |
| upgrade8    | `SplitListFanoutAsync`        | ❌                       | ✅                       | Parallelism and minimal sync overhead                                    | Async fan-out model using memory isolation to reduce locking contention  |


# Testing Focus

### ✅ Core Functional Tests

| Test Name                        | Purpose                                                             |
|----------------------------------|---------------------------------------------------------------------|
| `test_basic_insertion_and_expiry` | Verify that items inserted with specific timestamps expire correctly. |
| `test_expire_front_clears_bucket` | Ensure the front bucket is expired and cleared correctly.           |
| `test_ignore_out_of_range_items` | Confirm out-of-range insertions are ignored safely.                 |
| `test_len_and_is_empty_consistency` | Validate length and empty state tracking.                      |
| `test_multiple_items_same_bucket` | Insert multiple items in one bucket and confirm grouped expiration. |
| `test_values_snapshot_consistency` | Ensure values returned reflect accurate snapshot of stored IDs.    |


### ✅ Edge Case Tests

| Test Name                                      | Purpose                                                                 |
|-----------------------------------------------|-------------------------------------------------------------------------|
| `test_zero_items_expire_empty`                | Ensure calling expire on empty buckets returns an empty vector.        |
| `test_expire_all_buckets_and_reuse`           | Verify that buckets recycle properly over time.                         |
| `test_insert_exactly_on_boundary`             | Validate that boundary-timed insertions go to the correct bucket.       |
| `test_duplicate_insert_overwrite`             | Check that reinsertion updates the expiration bucket.                   |
| `test_len_decreases_after_expiry`             | Confirm that expired items reduce the length appropriately.             |
| `test_insertion_of_duplicate_items_different_ids` | Ensure two unique keys don’t interfere even with similar data.     |
| `test_insert_then_immediate_expire_front`     | Confirm immediate expiry is accurate for short-lived entries.           |
| `test_all_buckets_empty_no_panics`            | Rotate through multiple empty buckets and ensure no panics occur.       |
| `test_high_volume_single_bucket`              | Flood a single bucket to ensure handling of volume is correct.          |
| `test_expire_when_all_buckets_expired`        | Confirm expire still returns empty even when all data has expired.      |


## ✅ Concurrency Safety Tests

These tests help validate the thread safety and race condition resilience of expiry data structures.

| Test Name                                   | Purpose                                                                                           | Applicability       |
|--------------------------------------------|---------------------------------------------------------------------------------------------------|---------------------|
| `test_concurrent_inserts`                  | Insert from multiple threads and ensure correctness.                                              | ✅ Thread-safe only |
| `test_concurrent_inserts_stressed`         | High-volume inserts from multiple threads to stress test thread safety.                          | ✅ Thread-safe only |
| `test_concurrent_expire_and_insert`        | Simultaneous insert and expiry operations should be race-free.                                   | ✅ Thread-safe only |
| `test_concurrent_expire_and_insert_stressed` | Heavy simultaneous insertions and expirations to stress interaction safety.                    | ✅ Thread-safe only |
| `test_concurrent_reads`                    | Access values and length during concurrent mutation operations.                                   | ✅ Thread-safe only |
| `test_concurrent_reads_stressed`           | Frequent reads across many threads while writes are ongoing.                                     | ✅ Thread-safe only |
| `test_concurrent_duplicate_inserts`        | Multiple threads insert the same key—ensure overwrite safety and idempotent expiry behavior.     | ✅ Thread-safe only |
| `test_concurrent_duplicate_inserts_stressed` | High-volume redundant insertions from multiple threads.                                         | ✅ Thread-safe only |
| `test_concurrent_len_and_is_empty`         | Concurrent `len()` and `is_empty()` calls during mutation to test read stability.                | ✅ Thread-safe only |
| `test_concurrent_len_and_is_empty_stressed` | Stress-test of `len()` and `is_empty()` during heavy concurrent writes/expirations.            | ✅ Thread-safe only |
| `test_concurrent_tick_overlap`             | Multiple threads call `tick()` concurrently for same or overlapping timestamps.                  | ✅ Thread-safe only |
| `test_concurrent_tick_overlap_stressed`    | Many threads invoke overlapping `tick()` operations under stress.                                | ✅ Thread-safe only |
