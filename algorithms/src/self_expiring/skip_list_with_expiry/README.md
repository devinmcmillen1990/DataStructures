# Skip List with Expiry – **Concurrent Upgrade Matrix**

| **Upgrade** | **Struct Name**             | **Skip List Based?** | **Concurrency Support Strategy**        | **Motivation for Enhancement**                          | **Notes**                                              |
|-------------|-----------------------------|-----------------------|-----------------------------------------|----------------------------------------------------------|--------------------------------------------------------|
| `base`      | `SkipListExpiry`            | ✅                    | ❌ None (single-threaded only)          | Core structure using skip list for expiring elements     | Retained as a baseline for benchmarking                |
| `upgrade1`  | `LevelIndexedSkipList`      | ✅                    | ❌ Not internally safe (`Arc<Mutex<_>>`) | Improve time precision via level-indexed bucketing       | Can be used concurrently via wrapper                   |
| `upgrade2`  | `ConcurrentLevelSkipList`   | ✅                    | ✅ Built-in fine-grained locking         | Make level-indexed skip list thread-safe                 | Uses `RwLock` per bucket                               |
| `upgrade3`  | `BucketBTreeMapExpiry`      | ❌                    | ✅ `RwLock` guarded internal state       | Track order of expiry with thread safety                 | Combines BTreeMap and HashMap with locking             |
| `upgrade4`  | `SplitListLinkedBuckets`    | ❌                    | ✅ Built-in fine-grained locking         | Explore sequential data layout for expiry                | Linked-bucket memory layout with expiry groups         |
| `upgrade5`  | `SplitListFanoutAsync`      | ❌                    | ✅ Async isolation (task-local memory)   | Maximize throughput via isolated fanout tasks            | Lock-free async model with fan-out workers             |

---
