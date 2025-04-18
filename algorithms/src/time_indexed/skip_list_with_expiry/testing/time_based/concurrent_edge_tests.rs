use crate::time_indexed::skip_list_with_expiry::traits::ConcurrentTimeBasedExpiry;
use chrono::Utc;
use std::sync::Arc;

pub fn edge_test_zero_items_expire_empty<E>(expiry: Arc<E>)
where
    E: ConcurrentTimeBasedExpiry<String>,
{
    assert_eq!(expiry.expire_front(), Vec::<String>::new());
}

pub fn edge_test_insert_exactly_on_boundary<E>(expiry: Arc<E>)
where
    E: ConcurrentTimeBasedExpiry<String>,
{
    let timestamp = 1745013196;

    expiry.insert("A".to_string(), timestamp - 5);
    expiry.insert("B".to_string(), timestamp); // same bucket as A
    let expired = expiry.expire_front();

    assert!(expired.contains(&"A".to_string()));
    assert!(expired.contains(&"B".to_string()));
    assert_eq!(expired.len(), 2);
}

pub fn edge_test_duplicate_insert_overwrite<E>(expiry: Arc<E>)
where
    E: ConcurrentTimeBasedExpiry<String>,
{
    let timestamp1 = 1745013191;
    let timestamp2 = 1745013197;

    expiry.insert("X".to_string(), timestamp1);
    expiry.insert("X".to_string(), timestamp2);
    let expired = expiry.expire_front();

    assert_eq!(expired, vec!["X".to_string()]);
}

pub fn edge_test_expire_all_buckets_and_reuse<E>(expiry: Arc<E>)
where
    E: ConcurrentTimeBasedExpiry<String>,
{
    let base = Utc::now().timestamp();

    for i in 0..3 {
        expiry.insert(format!("Item-{}", i), base + i);
        expiry.expire_front();
    }

    for i in 3..6 {
        expiry.insert(format!("Item-{}", i), base + i);
    }

    let mut expired_total = vec![];
    for _ in 0..3 {
        expired_total.extend(expiry.expire_front());
    }

    assert!(expired_total.contains(&"Item-3".to_string()));
    assert!(expired_total.contains(&"Item-4".to_string()));
    assert!(expired_total.contains(&"Item-5".to_string()));
}

pub fn edge_test_len_decreases_after_expiry<E>(expiry: Arc<E>)
where
    E: ConcurrentTimeBasedExpiry<String>,
{
    let now = Utc::now().timestamp();
    expiry.insert("Item-A".to_string(), now);
    expiry.insert("Item-B".to_string(), now);

    let _ = expiry.expire_front();
    let _ = expiry.expire_front();

    assert_eq!(expiry.len(), 0);
    assert!(expiry.is_empty());
}
