use crate::self_expiring::skip_list_with_expiry::upgrade3::bucket_b_tree_map_expiry::BucketBTreeMapExpiry;
use crate::self_expiring::testing::time_based::concurrent_core_tests::{
    core_test_basic_insertion_and_expiry, core_test_expire_front_clears_bucket,
    core_test_len_and_is_empty_consistency, core_test_multiple_items_same_bucket,
    core_test_values_snapshot_consistency,
};
use crate::self_expiring::testing::time_based::concurrent_edge_tests::{
    edge_test_duplicate_insert_overwrite, edge_test_expire_all_buckets_and_reuse,
    edge_test_len_decreases_after_expiry, edge_test_zero_items_expire_empty,
};
use crate::self_expiring::testing::time_based::concurrent_threading_stress_tests::{
    threading_stress_test_concurrent_expiry_behavior, threading_stress_test_concurrent_insertions,
    threading_stress_test_concurrent_mixed_read_write,
};
use crate::self_expiring::testing::time_based::concurrent_threading_tests::{
    threading_test_concurrent_expiry_behavior, threading_test_concurrent_insertions,
    threading_test_concurrent_mixed_read_write,
};
use std::sync::Arc;

/**
 * CORE TESTS
 */
#[test]
fn test_basic_insertion_and_expiry() {
    let expiry = Arc::new(BucketBTreeMapExpiry::new());
    core_test_basic_insertion_and_expiry(expiry)
}

#[test]
fn test_expire_front_clears_bucket() {
    let expiry = Arc::new(BucketBTreeMapExpiry::new());
    core_test_expire_front_clears_bucket(expiry)
}

// TODO: Provide description why this is not applicable
#[test]
fn test_ignore_out_of_range_items() {
    //let expiry = Arc::new(BucketBTreeMapExpiry::new());
    //core_test_ignore_out_of_range_items(expiry)

    // Not Applicable
}

#[test]
fn test_len_and_is_empty_consistency() {
    let expiry = Arc::new(BucketBTreeMapExpiry::new());
    core_test_len_and_is_empty_consistency(expiry)
}

#[test]
fn test_multiple_items_same_bucket() {
    let expiry = Arc::new(BucketBTreeMapExpiry::new());
    core_test_multiple_items_same_bucket(expiry)
}

#[test]
fn test_values_snapshot_consistency() {
    let expiry = Arc::new(BucketBTreeMapExpiry::new());
    core_test_values_snapshot_consistency(expiry)
}

/**
 * EDGE-CASE TESTS
 */
#[test]
fn test_zero_items_expire_empty() {
    let expiry = Arc::new(BucketBTreeMapExpiry::new());
    edge_test_zero_items_expire_empty(expiry)
}

#[test]
fn test_expire_all_buckets_and_reuse() {
    let expiry = Arc::new(BucketBTreeMapExpiry::new());
    edge_test_expire_all_buckets_and_reuse(expiry)
}

#[test]
fn test_len_decreases_after_expiry() {
    let expiry = Arc::new(BucketBTreeMapExpiry::new());
    edge_test_len_decreases_after_expiry(expiry)
}

#[test]
fn test_duplicate_insert_overwrite() {
    let expiry = Arc::new(BucketBTreeMapExpiry::new());
    edge_test_duplicate_insert_overwrite(expiry)
}

// TODO: Provide description why this is not applicable
#[test]
fn test_insert_and_expire_mixed_order() {
    // Not Applicable
}

// TODO: Provide description why this is not applicable
#[test]
fn test_expire_partial_and_continue() {
    // Not Applicable
}

// TODO: Provide description why this is not applicable
#[test]
fn test_reschedule_existing_item() {
    // Not Applicable
}

/**
 * THREADING TESTS
 */
#[test]
fn test_concurrent_insertions() {
    let expiry = Arc::new(BucketBTreeMapExpiry::new());
    threading_test_concurrent_insertions(expiry);
}

#[test]
fn test_concurrent_expiry_behavior() {
    let expiry = Arc::new(BucketBTreeMapExpiry::new());
    threading_test_concurrent_expiry_behavior(expiry);
}

#[test]
fn test_concurrent_mixed_read_write() {
    let expiry = Arc::new(BucketBTreeMapExpiry::new());
    threading_test_concurrent_mixed_read_write(expiry);
}

#[test]
#[ignore = "stress test"]
fn test_concurrent_insertions_stressed() {
    let expiry = Arc::new(BucketBTreeMapExpiry::new());
    threading_stress_test_concurrent_insertions(expiry);
}

#[test]
#[ignore = "stress test"]
fn test_concurrent_expiry_behavior_stressed() {
    let expiry = Arc::new(BucketBTreeMapExpiry::new());
    threading_stress_test_concurrent_expiry_behavior(expiry);
}

#[test]
#[ignore = "stress test"]
fn test_concurrent_mixed_read_write_stressed() {
    let expiry = Arc::new(BucketBTreeMapExpiry::new());
    threading_stress_test_concurrent_mixed_read_write(expiry);
}
