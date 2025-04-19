use crate::self_expiring::skip_list_with_expiry::upgrade1::level_indexed_skip_list::LevelIndexedSkipList;
use crate::self_expiring::skip_list_with_expiry::testing::index_based::core_tests::{
    core_test_basic_insertion_and_expiry, core_test_expire_front_clears_bucket,
    core_test_ignore_out_of_range_items, core_test_len_and_is_empty_consistency,
    core_test_multiple_items_same_bucket, core_test_values_snapshot_consistency,
};
use crate::self_expiring::skip_list_with_expiry::testing::index_based::edge_tests::{
    edge_test_duplicate_insert_overwrite, edge_test_expire_all_buckets_and_reuse,
    edge_test_insert_exactly_on_boundary, edge_test_len_decreases_after_expiry,
    edge_test_zero_items_expire_empty,
};

/**
 * CORE TESTS
 */
#[test]
fn test_basic_insertion_and_expiry() {
    core_test_basic_insertion_and_expiry(LevelIndexedSkipList::new(3))
}

#[test]
fn test_expire_front_clears_bucket() {
    core_test_expire_front_clears_bucket(LevelIndexedSkipList::new(3))
}

#[test]
fn test_ignore_out_of_range_items() {
    core_test_ignore_out_of_range_items(LevelIndexedSkipList::new(2))
}

#[test]
fn test_len_and_is_empty_consistency() {
    core_test_len_and_is_empty_consistency(LevelIndexedSkipList::new(3))
}

#[test]
fn test_multiple_items_same_bucket() {
    core_test_multiple_items_same_bucket(LevelIndexedSkipList::new(2))
}

#[test]
fn test_values_snapshot_consistency() {
    core_test_values_snapshot_consistency(LevelIndexedSkipList::new(3))
}

/**
 * EDGE-CASE TESTS
 */
#[test]
fn test_zero_items_expire_empty() {
    edge_test_zero_items_expire_empty(LevelIndexedSkipList::new(3))
}

#[test]
fn test_expire_all_buckets_and_reuse() {
    edge_test_expire_all_buckets_and_reuse(LevelIndexedSkipList::new(2))
}

#[test]
fn test_len_decreases_after_expiry() {
    edge_test_len_decreases_after_expiry(LevelIndexedSkipList::new(4))
}

#[test]
fn test_insert_exactly_on_boundary() {
    edge_test_insert_exactly_on_boundary(LevelIndexedSkipList::new(4))
}

#[test]
fn test_duplicate_insert_overwrite() {
    edge_test_duplicate_insert_overwrite(LevelIndexedSkipList::new(3))
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
// TODO: Provide description why this is not applicable
#[test]
fn test_concurrent_insertions() {
    // Not Applicable
}

// TODO: Provide description why this is not applicable
#[test]
fn test_concurrent_expiry_behavior() {
    // Not Applicable
}

/**
 * THREADING STRESS TESTS
 */
// TODO: Provide description why this is not applicable
#[test]
#[ignore = "stress test"]
fn test_concurrent_insertions_stressed() {
    // Not Applicable
}

// TODO: Provide description why this is not applicable
#[test]
#[ignore = "stress test"]
fn test_concurrent_expiry_behavior_stressed() {
    // Not Applicable
}

// TODO: Provide description why this is not applicable
#[test]
#[ignore = "stress test"]
fn test_concurrent_mixed_read_write_stressed() {
    // Not Applicable
}
