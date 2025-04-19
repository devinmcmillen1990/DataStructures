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

These tests validate multi-threaded behavior, race resistance, and safe concurrent access patterns.

| **Test Name**                               | **Purpose**                                                                                     |
|--------------------------------------------|--------------------------------------------------------------------------------------------------|
| `test_concurrent_inserts`                  | Insert from multiple threads and ensure data integrity.                                          |
| `test_concurrent_inserts_stressed`         | High-volume concurrent inserts to simulate stress conditions.                                   |
| `test_concurrent_expire_and_insert`        | Simultaneous insertions and expirations to detect race conditions.                              |
| `test_concurrent_expire_and_insert_stressed` | Stress test for concurrent inserts and expirations at high volume.                           |
| `test_concurrent_reads`                    | Validate safe concurrent access to `values()` and `len()` during mutations.                     |
| `test_concurrent_reads_stressed`           | Repeated concurrent reads during heavy mutations.                                               |
| `test_concurrent_duplicate_inserts`        | Test concurrent overwriting of identical keys for idempotency and data safety.                  |
| `test_concurrent_duplicate_inserts_stressed` | Simulate high-frequency concurrent insertions of the same key.                               |
| `test_concurrent_len_and_is_empty`         | Ensure consistency of `len()` and `is_empty()` during active concurrent changes.                |
| `test_concurrent_len_and_is_empty_stressed` | Repeated checks of `len()`/`is_empty()` while concurrent operations run.                     |
| `test_concurrent_tick_overlap`             | Validate correctness of overlapping `tick()` operations from multiple threads.                  |
| `test_concurrent_tick_overlap_stressed`    | Stress test for `tick()` logic under concurrent scheduling.                                     |
