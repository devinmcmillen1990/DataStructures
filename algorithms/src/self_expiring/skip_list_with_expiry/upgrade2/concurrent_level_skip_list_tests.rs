use crate::self_expiring::skip_list_with_expiry::upgrade2::concurrent_level_skip_list::ConcurrentLevelSkipList;
use crate::self_expiring::skip_list_with_expiry::testing::index_based::concurrent_threading_stress_tests::{
    threading_stress_test_concurrent_expiry_behavior, threading_stress_test_concurrent_insertions,
    threading_stress_test_concurrent_mixed_read_write,
};
use crate::self_expiring::skip_list_with_expiry::testing::index_based::concurrent_threading_tests::{
    threading_test_concurrent_expiry_behavior, threading_test_concurrent_insertions,
    threading_test_concurrent_mixed_read_write,
};
use std::sync::Arc;

// TODO: Implement Core Tests
// TODO: Implement Edge-Case Tests

/**
 * THREADING TESTS
 */
#[test]
fn test_concurrent_insertions() {
    let expiry = Arc::new(ConcurrentLevelSkipList::new(8));
    threading_test_concurrent_insertions(expiry);
}

#[test]
fn test_concurrent_expiry_behavior() {
    let expiry = Arc::new(ConcurrentLevelSkipList::new(8));
    threading_test_concurrent_expiry_behavior(expiry);
}

#[test]
fn test_concurrent_mixed_read_write() {
    let expiry = Arc::new(ConcurrentLevelSkipList::new(8));
    threading_test_concurrent_mixed_read_write(expiry);
}

#[test]
#[ignore = "stress test"]
fn test_concurrent_insertions_stressed() {
    let expiry = Arc::new(ConcurrentLevelSkipList::new(10));
    threading_stress_test_concurrent_insertions(expiry);
}

#[test]
#[ignore = "stress test"]
fn test_concurrent_expiry_behavior_stressed() {
    let expiry = Arc::new(ConcurrentLevelSkipList::new(10));
    threading_stress_test_concurrent_expiry_behavior(expiry);
}

#[test]
#[ignore = "stress test"]
fn test_concurrent_mixed_read_write_stressed() {
    let expiry = Arc::new(ConcurrentLevelSkipList::new(10));
    threading_stress_test_concurrent_mixed_read_write(expiry);
}
