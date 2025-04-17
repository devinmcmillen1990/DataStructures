use crate::time_indexed::skip_list_with_expiry::traits::TimeBasedExpiry;
use chrono::Utc;

// TODO: Double check the <String> -> Replace with secondary wild card
// TODO: Add cfg(test) decorators
// TODO: Add Docu-Comments for each test describing the intent and operations.

pub fn edge_test_zero_items_expire_empty<E>(mut expiry: E)
where
    E: TimeBasedExpiry<String>,
{
    assert_eq!(expiry.expire_front(), Vec::<String>::new());
}

pub fn edge_test_insert_exactly_on_boundary<E>(mut expiry: E)
where
    E: TimeBasedExpiry<String>,
{
    expiry.insert("B".to_string(), Utc::now().timestamp() + 5);
    expiry.expire_front(); // bucket 0
    let expired = expiry.expire_front(); // bucket 1
    assert_eq!(expired, vec!["B".to_string()]);
}

pub fn edge_test_duplicate_insert_overwrite<E>(mut expiry: E)
where
    E: TimeBasedExpiry<String>,
{
    let now = Utc::now().timestamp();

    expiry.insert("X".to_string(), now);
    assert_eq!(expiry.expire_front(), vec!["X".to_string()]);
    expiry.expire_front();

    let new_time = now + 6;
    expiry.insert("X".to_string(), new_time);
    expiry.expire_front();
    let expired = expiry.expire_front();
    assert_eq!(expired, vec!["X".to_string()]);
}

pub fn edge_test_expire_all_buckets_and_reuse<E>(mut expiry: E)
where
    E: TimeBasedExpiry<String>,
{
    let now = Utc::now().timestamp();

    expiry.insert("T".to_string(), now);
    assert_eq!(expiry.expire_front(), vec!["T".to_string()]);
    expiry.expire_front();
    expiry.expire_front();

    let new_time = now + 15;
    expiry.insert("U".to_string(), new_time);
    let expired = expiry.expire_front();
    assert_eq!(expired, vec!["U".to_string()]);
}

pub fn edge_test_len_decreases_after_expiry<E>(mut expiry: E)
where
    E: TimeBasedExpiry<String>,
{
    let now = Utc::now().timestamp();

    expiry.insert("A".to_string(), now);
    expiry.insert("B".to_string(), now + 2);
    expiry.insert("C".to_string(), now + 4);

    assert_eq!(expiry.len(), 3);
    expiry.expire_front();
    assert_eq!(expiry.len(), 2);
    expiry.expire_front();
    assert_eq!(expiry.len(), 1);
    expiry.expire_front();
    assert_eq!(expiry.len(), 0);
}

// pub fn edge_test_insert_and_expire_mixed_order<E>(mut expiry: E)
// where
//     E: TimeBasedExpiry<String>,
// {
//     // TODO: write out implementation
// }

// pub fn edge_test_expire_partial_and_continue<E>(mut expiry: E)
// where
//     E: TimeBasedExpiry<String>,
// {
//     // TODO: write out implementation
// }

// pub fn edge_test_reschedule_existing_item<E>(mut expiry: E)
// where
//     E: TimeBasedExpiry<String>,
// {
//     // TODO: write out implementation
// }
