# Skip List with Expiry – **Concurrent Upgrade Matrix**

This document outlines the concurrent-ready enhancements for skip-list-based and bucket-based expiry algorithms. Non-concurrent prototypes (like `upgrade3` and `upgrade4`) have been removed or archived separately for reference.

| **Upgrade** | **Struct Name**             | **Skip List Based?** | **Concurrency Support Strategy**        | **Motivation for Enhancement**                          | **Notes**                                              |
|-------------|-----------------------------|-----------------------|-----------------------------------------|----------------------------------------------------------|--------------------------------------------------------|
| `base`      | `SkipListExpiry`            | ✅                    | ❌ None (single-threaded only)          | Core structure using skip list for expiring elements     | Retained as a baseline for benchmarking                |
| `upgrade1`  | `LevelIndexedSkipList`      | ✅                    | ❌ Not internally safe (`Arc<Mutex<_>>`) | Improve time precision via level-indexed bucketing       | Can be used concurrently via wrapper                   |
| `upgrade2`  | `ConcurrentLevelSkipList`   | ✅                    | ✅ Built-in fine-grained locking         | Make level-indexed skip list thread-safe                 | Uses `RwLock` per bucket                               |
| `upgrade3`  | `BucketBTreeMapExpiry`      | ❌                    | ✅ `RwLock` guarded internal state       | Track order of expiry with thread safety                 | Combines BTreeMap and HashMap with locking             |
| `upgrade4`  | `SplitListLinkedBuckets`    | ❌                    | ✅ Built-in fine-grained locking         | Explore sequential data layout for expiry                | Linked-bucket memory layout with expiry groups         |
| `upgrade5`  | `SplitListFanoutAsync`      | ❌                    | ✅ Async isolation (task-local memory)   | Maximize throughput via isolated fanout tasks            | Lock-free async model with fan-out workers             |

---

### 🔒 Archived Prototypes

These are maintained for internal testing and exploration, but are not considered production-safe due to lack of concurrency support:

- `SkipListFlatWheel`
- `BucketHashSetExpiry`
- `SplitListTrieIndexed`

They may still be valuable for benchmarking and educational purposes under `/prototypes`.
