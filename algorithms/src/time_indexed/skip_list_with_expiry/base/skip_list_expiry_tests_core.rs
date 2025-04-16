use crate::time_indexed::skip_list_with_expiry::base::skip_list_expiry::SkipListExpiry;
use chrono::Utc;

#[test]
fn test_basic_insertion_and_expiry() {
    let skiplist = SkipListExpiry::new(3, 1);
    let now = Utc::now().timestamp();

    skiplist.insert("A".to_string(), now);
    let expired = skiplist.expire_front();

    assert_eq!(expired, vec!["A"]);
}

#[test]
fn test_expire_front_clears_bucket() {
    let skiplist = SkipListExpiry::new(3, 1);
    let now = Utc::now().timestamp();

    skiplist.insert("A".to_string(), now);
    assert_eq!(skiplist.expire_front(), vec!["A"]);
    assert!(skiplist.expire_front().is_empty());
}

#[test]
fn test_ignore_out_of_range_items() {
    let skiplist = SkipListExpiry::new(2, 1);
    let now = Utc::now().timestamp();

    skiplist.insert("TooEarly".to_string(), now - 100);
    skiplist.insert("TooLate".to_string(), now + 100);

    assert_eq!(skiplist.len(), 0);
    assert!(skiplist.is_empty());
}

#[test]
fn test_len_and_is_empty_consistency() {
    let skiplist = SkipListExpiry::new(3, 1);
    let now = Utc::now().timestamp();

    assert!(skiplist.is_empty());

    skiplist.insert("A".to_string(), now);
    assert_eq!(skiplist.len(), 1);
    assert!(!skiplist.is_empty());

    skiplist.expire_front();
    assert_eq!(skiplist.len(), 0);
    assert!(skiplist.is_empty());
}

#[test]
fn test_multiple_items_same_bucket() {
    let skiplist = SkipListExpiry::new(2, 10);
    let now = Utc::now().timestamp();

    skiplist.insert("X".to_string(), now);
    skiplist.insert("Y".to_string(), now);
    skiplist.insert("Z".to_string(), now);

    let expired = skiplist.expire_front();
    assert_eq!(expired.len(), 3);
    assert!(expired.contains(&"X".to_string()));
    assert!(expired.contains(&"Y".to_string()));
    assert!(expired.contains(&"Z".to_string()));
}

#[test]
fn test_values_snapshot_consistency() {
    let skiplist = SkipListExpiry::new(3, 1);
    let now = Utc::now().timestamp();

    skiplist.insert("M".to_string(), now);
    skiplist.insert("N".to_string(), now + 1);
    skiplist.insert("O".to_string(), now + 2);

    let values = skiplist.values();
    assert_eq!(values.len(), 3);
    assert!(values.contains(&"M".to_string()));
    assert!(values.contains(&"N".to_string()));
    assert!(values.contains(&"O".to_string()));
}
