use crate::self_expiring::split_list_with_expiry::traits::ConcurrentTimeBasedExpiry;
use std::sync::Arc;
use std::time::{SystemTime, UNIX_EPOCH};

pub fn core_test_basic_insertion_and_expiry<E>(expiry: Arc<E>)
where
    E: ConcurrentTimeBasedExpiry<String> + 'static,
{
    let now = current_time();
    expiry.insert("A".to_string(), now + 10);
    expiry.insert("B".to_string(), now + 20);

    assert_eq!(expiry.len(), 2);
    assert!(!expiry.is_empty());
}

pub fn core_test_expire_front_clears_bucket<E>(expiry: Arc<E>)
where
    E: ConcurrentTimeBasedExpiry<String> + 'static,
{
    let now = current_time();
    expiry.insert("A".to_string(), now);
    let _ = expiry.expire_front();

    assert_eq!(expiry.len(), 0);
    assert!(expiry.is_empty());
}

pub fn core_test_len_and_is_empty_consistency<E>(expiry: Arc<E>)
where
    E: ConcurrentTimeBasedExpiry<String> + 'static,
{
    let now = current_time();
    assert_eq!(expiry.len(), 0);
    assert!(expiry.is_empty());

    expiry.insert("Z".to_string(), now);
    assert_eq!(expiry.len(), 1);
    assert!(!expiry.is_empty());

    let _ = expiry.expire_front();
    assert_eq!(expiry.len(), 0);
    assert!(expiry.is_empty());
}

pub fn core_test_multiple_items_same_bucket<E>(expiry: Arc<E>)
where
    E: ConcurrentTimeBasedExpiry<String> + 'static,
{
    let now = current_time();
    expiry.insert("A".to_string(), now);
    expiry.insert("B".to_string(), now);
    expiry.insert("C".to_string(), now);

    let mut expired = expiry.expire_front();
    expired.sort();
    
    assert_eq!(expired, vec!["A", "B", "C"]);
}

pub fn core_test_values_snapshot_consistency<E>(expiry: Arc<E>)
where
    E: ConcurrentTimeBasedExpiry<String> + 'static,
{
    let now = current_time();
    expiry.insert("Snap".to_string(), now + 50);
    let snapshot = expiry.values();

    assert_eq!(snapshot, vec!["Snap"]);
}

fn current_time() -> i64 {
    SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs() as i64
}
