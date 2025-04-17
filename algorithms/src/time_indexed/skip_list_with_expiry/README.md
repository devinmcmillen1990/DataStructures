# Skip List with Expiry – Upgrade Matrix

This document outlines the various upgrades and design variations for skip-list-based and bucket-based expiry algorithms.

| **Upgrade** | **Struct Name**               | **Skip List Based?** | **Concurrency Support Strategy**           | **Motivation for Enhancement**                                            | **Notes**                                                                 |
|-------------|-------------------------------|-----------------------|--------------------------------------------|---------------------------------------------------------------------------|---------------------------------------------------------------------------|
| `base`      | `SkipListExpiry`              | ✅                    | None (single-threaded only)                | Core structure using skip list for expiring elements                     | Simple and performant for single-threaded use                             |
| `upgrade1`  | `LevelIndexedSkipList`        | ✅                    | Not internally safe (wrap with `Mutex`)    | Improve time precision via level-indexed bucketing                       | Can be used concurrently via `Arc<Mutex<_>>` wrapper                     |
| `upgrade2`  | `SkipListConcurrentLevel`     | ✅                    | Built-in fine-grained locking              | Make level-indexed skip list thread-safe                                 | Internal synchronization per bucket or structure                         |
| `upgrade3`  | `SkipListFlatWheel`           | ✅                    | None (simplified structure)                | Reduce memory & simplify design                                          | Dropped internal safety for compactness                                  |
| `upgrade4`  | `BucketHashSetExpiry`         | ❌                    | None                                       | Improve insertion speed and simplicity                                   | Uses unordered buckets for fast writes                                   |
| `upgrade5`  | `BucketBTreeMapExpiry`        | ❌                    | `RwLock` guarded internal state            | Track order of expiry with thread safety                                 | Combines BTreeMap and HashMap + locking                                  |
| `upgrade6`  | `SplitListLinkedBuckets`      | ❌                    | Built-in fine-grained locking              | Explore sequential data layout for expiry                                | Linked-bucket memory layout with expiry groups                           |
| `upgrade7`  | `SplitListTrieIndexed`        | ❌                    | None                                       | Scale indexed lookups in large systems                                   | Trie-based structure not safe for concurrency                            |
| `upgrade8`  | `SplitListFanoutAsync`        | ❌                    | Async isolation (task-local memory)        | Maximize throughput via isolated fanout tasks                            | Low-lock async model with message-passing                                |


