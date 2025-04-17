use crate::time_indexed::skip_list_with_expiry::traits::IndexBasedExpiry;

// TODO: Double check the <String> -> Replace with secondary wild card
// TODO: Add cfg(test) decorators
// TODO: Add Docu-Comments for each test describing the intent and operations.

pub fn edge_test_zero_items_expire_empty<E>(mut expiry: E)
where
    E: IndexBasedExpiry<String>,
{
    let expired = expiry.expire_front();
    assert!(expired.is_empty());
}

// TODO: Setup last_level to get the length of the expiry
//          - Then have a loop expire for the (length - 1)
//          - Then expire front and check
pub fn edge_test_insert_exactly_on_boundary<E>(mut expiry: E)
where
    E: IndexBasedExpiry<String>,
{
    let last_level = 3;
    expiry.insert("EdgeCase".to_string(), last_level);

    let expired0 = expiry.expire_front();
    let expired1 = expiry.expire_front();
    let expired = expiry.expire_front();

    assert_eq!(expired, Vec::<String>::new());
}

pub fn edge_test_duplicate_insert_overwrite<E>(mut expiry: E)
where
    E: IndexBasedExpiry<String>,
{
    expiry.insert("X".to_string(), 0);
    expiry.insert("X".to_string(), 2);

    let first = expiry.expire_front();
    assert!(!first.is_empty());
    print!("\n{:?}\n\n", first);

    let second = expiry.expire_front();
    assert_eq!(second, Vec::<String>::new());
}

// TODO: Setup last_level to get the length of the expiry
//          - Then have a loop expire for the (length)
//          - Then insert, expire front and check
pub fn edge_test_expire_all_buckets_and_reuse<E>(mut expiry: E)
where
    E: IndexBasedExpiry<String>,
{
    expiry.insert("T".to_string(), 0);
    assert_eq!(expiry.expire_front(), vec!["T".to_string()]);
    expiry.expire_front();
    expiry.expire_front();

    expiry.insert("U".to_string(), 0);
    let expired = expiry.expire_front();
    assert_eq!(expired, vec!["U".to_string()]);
}

pub fn edge_test_len_decreases_after_expiry<E>(mut expiry: E)
where
    E: IndexBasedExpiry<String>,
{
    expiry.insert("L0".to_string(), 0);
    expiry.insert("L1".to_string(), 1);
    expiry.insert("L2".to_string(), 2);

    let e0 = expiry.expire_front();
    assert_eq!(e0, vec!["L0".to_string()]);
    let e1 = expiry.expire_front();
    assert_eq!(e1, vec!["L1".to_string()]);
    let e2 = expiry.expire_front();
    assert_eq!(e2, vec!["L2".to_string()]);

    assert!(expiry.is_empty());
}

pub fn edge_test_insert_and_expire_mixed_order<E>(mut expiry: E)
where
    E: IndexBasedExpiry<String>,
{
    expiry.insert("C".to_string(), 2);
    expiry.insert("A".to_string(), 0);
    expiry.insert("B".to_string(), 1);

    let e0 = expiry.expire_front();
    assert_eq!(e0, vec!["A".to_string()]);
    let e1 = expiry.expire_front();
    assert_eq!(e1, vec!["B".to_string()]);
    let e2 = expiry.expire_front();
    assert_eq!(e2, vec!["C".to_string()]);
}

pub fn edge_test_expire_partial_and_continue<E>(mut expiry: E)
where
    E: IndexBasedExpiry<String>,
{
    expiry.insert("OnlyOne".to_string(), 2);

    // Expire two empty levels
    assert!(expiry.expire_front().is_empty());
    assert!(expiry.expire_front().is_empty());

    // Third one should succeed
    let expired = expiry.expire_front();
    assert_eq!(expired, vec!["OnlyOne".to_string()]);
}

pub fn edge_test_reschedule_existing_item<E>(mut expiry: E)
where
    E: IndexBasedExpiry<String>,
{
    expiry.insert("X".to_string(), 0);
    expiry.insert("X".to_string(), 2); // reschedule before level 0 expires

    // Expire level 0 – should be empty
    assert!(expiry.expire_front().is_empty());

    // Expire level 1 – still empty
    assert!(expiry.expire_front().is_empty());

    // Expire level 2 – should contain X
    let expired = expiry.expire_front();
    assert_eq!(expired, vec!["X".to_string()]);
}
