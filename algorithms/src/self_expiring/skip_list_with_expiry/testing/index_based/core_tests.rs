use crate::self_expiring::skip_list_with_expiry::traits::IndexBasedExpiry;

// TODO: Double check the <String> -> Replace with secondary wild card
// TODO: Add cfg(test) decorators
// TODO: Add Docu-Comments for each test describing the intent and operations.

pub fn core_test_basic_insertion_and_expiry<E>(mut expiry: E)
where
    E: IndexBasedExpiry<String>,
{
    expiry.insert("A".to_string(), 0);
    let expired = expiry.expire_front();
    assert_eq!(expired, vec!["A"]);
}

pub fn core_test_expire_front_clears_bucket<E>(mut expiry: E)
where
    E: IndexBasedExpiry<String>,
{
    expiry.insert("A".to_string(), 0);
    assert_eq!(expiry.expire_front(), vec!["A"]);
    assert!(expiry.expire_front().is_empty());
}

// TODO: Out of range goes into final bucket
pub fn core_test_ignore_out_of_range_items<E>(mut expiry: E)
where
    E: IndexBasedExpiry<String>,
{
    expiry.insert("TooHigh".to_string(), 100);
    expiry.insert("TooHigh2".to_string(), 5);

    assert_eq!(expiry.len(), 2);
    assert!(!expiry.is_empty());
}

pub fn core_test_len_and_is_empty_consistency<E>(mut expiry: E)
where
    E: IndexBasedExpiry<String>,
{
    assert!(expiry.is_empty());

    expiry.insert("A".to_string(), 1);
    assert_eq!(expiry.len(), 1);
    assert!(!expiry.is_empty());

    expiry.expire_front();
    expiry.expire_front();

    assert_eq!(expiry.len(), 0);
    assert!(expiry.is_empty());
}

pub fn core_test_multiple_items_same_bucket<E>(mut expiry: E)
where
    E: IndexBasedExpiry<String>,
{
    expiry.insert("X".to_string(), 0);
    expiry.insert("Y".to_string(), 0);
    expiry.insert("Z".to_string(), 0);

    let expired = expiry.expire_front();
    assert_eq!(expired.len(), 3);
    assert!(expired.contains(&"X".to_string()));
    assert!(expired.contains(&"Y".to_string()));
    assert!(expired.contains(&"Z".to_string()));
    assert!(expiry.is_empty());
}

pub fn core_test_values_snapshot_consistency<E>(mut expiry: E)
where
    E: IndexBasedExpiry<String>,
{
    expiry.insert("M".to_string(), 0);
    expiry.insert("N".to_string(), 1);
    expiry.insert("O".to_string(), 2);

    let values = expiry.values();
    assert_eq!(values.len(), 3);
    assert!(values.contains(&"M".to_string()));
    assert!(values.contains(&"N".to_string()));
    assert!(values.contains(&"O".to_string()));
}
