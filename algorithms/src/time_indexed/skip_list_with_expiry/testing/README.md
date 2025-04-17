## ✅ Skip List with Expiry Buckets - Test Coverage

### ✅ Core Functional Tests

| Test Name                             | Purpose                                                              |
|--------------------------------------|----------------------------------------------------------------------|
| `test_basic_insertion_and_expiry`    | Verifies items expire at the right time.                            |
| `test_expire_front_clears_bucket`    | Ensures buckets are recycled and cleared.                           |
| `test_ignore_out_of_range_items`     | Verifies that out-of-bounds insertions are ignored.                 |
| `test_len_and_is_empty_consistency`  | Confirms size tracking works with inserts and expirations.          |
| `test_multiple_items_same_bucket`    | Ensures grouping and batch expiration works per bucket.             |
| `test_values_snapshot_consistency`   | TODO-Fill this in                                                   |    

### ✅ Edge Case Tests

| Test Name                              | Purpose                                                              |
|---------------------------------------|----------------------------------------------------------------------|
| `test_zero_items_expire_empty`        | Calling expiration on an empty structure should return `[]`.        |
| `test_insert_exactly_on_boundary`     | Inserting at exact bucket boundary is handled correctly.            |
| `test_duplicate_insert_overwrite`     | Overwriting existing keys places them in the correct bucket.        |
| `test_expire_all_buckets_and_reuse`   | Ensures circular buffer rotates and remains correct.                |
| `test_len_decreases_after_expiry`     | TODO-Fill this in                                                   |
| `test_insert_and_expire_mixed_order`  | TODO-Fill this in                                                   |
| `test_expire_partial_and_continue`    | TODO-Fill this in                                                   |
| `test_reschedule_existing_item`       | TODO-Fill this in                                                   |

### 🚫 Concurrency Safety Tests

| Test Name                                     | Status           | Reason                                                 |
|----------------------------------------------|------------------|--------------------------------------------------------|
| `test_concurrent_insertions`                 | ❌ Not Applicable | Uses a global `Mutex`, no per-bucket locking.         |
| `test_concurrent_expiry_behavior`            | ❌ Not Applicable | Race conditions possible with `VecDeque`.             |
| `test_concurrent_insertions_stressed`        | ❌ Not Applicable | High-frequency concurrent insertions unsupported.     |
| `test_concurrent_expiry_behavior_stressed`   | ❌ Not Applicable | No support for stress-tested concurrent expirations.  |
| `test_concurrent_mixed_read_write_stressed`  | ❌ Not Applicable | Mixed workloads not synchronized with fine-grained locks. |
