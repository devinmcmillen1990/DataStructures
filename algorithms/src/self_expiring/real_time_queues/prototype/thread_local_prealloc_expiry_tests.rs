// use std::sync::Arc;

// use crate::self_expiring::real_time_queues::prototype::thread_local_prealloc_expiry::PrototypeThreadLocalExpiry;
// use crate::self_expiring::testing::time_based::concurrent_threading_stress_tests::{
//     threading_stress_test_concurrent_expiry_behavior, threading_stress_test_concurrent_insertions, 
//     threading_stress_test_concurrent_mixed_read_write
// };
// use crate::self_expiring::testing::time_based::concurrent_threading_tests::{
//     threading_test_concurrent_expiry_behavior, threading_test_concurrent_insertions, 
//     threading_test_concurrent_mixed_read_write
// };
// use crate::self_expiring::testing::time_based::concurrent_core_tests::{
//     core_test_basic_insertion_and_expiry, core_test_expire_front_clears_bucket,
//     core_test_len_and_is_empty_consistency, core_test_multiple_items_same_bucket, 
//     core_test_values_snapshot_consistency,
// };
// use crate::self_expiring::testing::time_based::concurrent_edge_tests::{
//     edge_test_duplicate_insert_overwrite, edge_test_expire_all_buckets_and_reuse,
//     edge_test_len_decreases_after_expiry, edge_test_zero_items_expire_empty,
// };

// /**
//  * CORE TESTS
//  */
// #[test]
// fn test_basic_insertion_and_expiry() {
//     let optimal_threads = num_cpus::get();
//     core_test_basic_insertion_and_expiry(Arc::new(PrototypeThreadLocalExpiry::new(optimal_threads)))
// }

// #[test]
// fn test_expire_front_clears_bucket() {
//     let optimal_threads = num_cpus::get();
//     core_test_expire_front_clears_bucket(Arc::new(PrototypeThreadLocalExpiry::new(optimal_threads)))
// }

// #[test]
// fn test_len_and_is_empty_consistency() {
//     let optimal_threads = num_cpus::get();
//     core_test_len_and_is_empty_consistency(Arc::new(PrototypeThreadLocalExpiry::new(optimal_threads)))
// }

// #[test]
// fn test_multiple_items_same_bucket() {
//     let optimal_threads = num_cpus::get();
//     core_test_multiple_items_same_bucket(Arc::new(PrototypeThreadLocalExpiry::new(optimal_threads)))
// }

// #[test]
// fn test_values_snapshot_consistency() {
//     let optimal_threads = num_cpus::get();
//     core_test_values_snapshot_consistency(Arc::new(PrototypeThreadLocalExpiry::new(optimal_threads)))
// }

// /**
//  * EDGE-CASE TESTS
//  */
// #[test]
// fn test_zero_items_expire_empty() {
//     let optimal_threads = num_cpus::get();
//     edge_test_zero_items_expire_empty(Arc::new(PrototypeThreadLocalExpiry::new(optimal_threads)))
// }

// #[test]
// fn test_expire_all_buckets_and_reuse() {
//     let optimal_threads = num_cpus::get();
//     edge_test_expire_all_buckets_and_reuse(Arc::new(PrototypeThreadLocalExpiry::new(optimal_threads)))
// }

// #[test]
// fn test_len_decreases_after_expiry() {
//     let optimal_threads = num_cpus::get();
//     edge_test_len_decreases_after_expiry(Arc::new(PrototypeThreadLocalExpiry::new(optimal_threads)))
// }

// #[test]
// fn test_duplicate_insert_overwrite() {
//     let optimal_threads = num_cpus::get();
//     edge_test_duplicate_insert_overwrite(Arc::new(PrototypeThreadLocalExpiry::new(optimal_threads)))
// }

// /**
//  * THREADING TESTS
//  */
// #[test]
// fn test_concurrent_insertions() {
//     let optimal_threads = num_cpus::get();
//     threading_test_concurrent_insertions(Arc::new(PrototypeThreadLocalExpiry::new(optimal_threads)))
// }

// #[test]
// fn test_concurrent_expiry_behavior() {
//     let optimal_threads = num_cpus::get();
//     threading_test_concurrent_expiry_behavior(Arc::new(PrototypeThreadLocalExpiry::new(optimal_threads)))
// }

// #[test]
// fn test_concurrent_mixed_read_write() {
//     let optimal_threads = num_cpus::get();
//     threading_test_concurrent_mixed_read_write(Arc::new(PrototypeThreadLocalExpiry::new(optimal_threads)))
// }

// /**
//  * THREADING STRESS TESTS
//  */
// #[test]
// #[ignore = "stress test"]
// fn test_concurrent_insertions_stressed() {
//     let optimal_threads = num_cpus::get();
//     threading_stress_test_concurrent_insertions(Arc::new(PrototypeThreadLocalExpiry::new(optimal_threads)))
// }

// #[test]
// #[ignore = "stress test"]
// fn test_concurrent_expiry_behavior_stressed() {
//     let optimal_threads = num_cpus::get();
//     threading_stress_test_concurrent_expiry_behavior(Arc::new(PrototypeThreadLocalExpiry::new(optimal_threads)))
// }

// #[test]
// #[ignore = "stress test"]
// fn test_concurrent_mixed_read_write_stressed() {
//     let optimal_threads = num_cpus::get();
//     threading_stress_test_concurrent_mixed_read_write(Arc::new(PrototypeThreadLocalExpiry::new(optimal_threads)))
// }
