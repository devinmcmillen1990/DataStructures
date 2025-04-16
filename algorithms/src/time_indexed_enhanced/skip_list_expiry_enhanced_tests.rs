use crate::time_indexed_enhanced::skip_list_expiry_enhanced::EnhancedSkipListExpiry;
use chrono::Utc;

#[test]
fn test_insert_with_expiry_ticks() {
    let now = Utc::now().timestamp() as usize;
    let skiplist = EnhancedSkipListExpiry::with_start_time(3, 1, now);

    skiplist.insert("A".to_string(), now);
    skiplist.insert("B".to_string(), now + 1);
    skiplist.insert("C".to_string(), now + 2);
    skiplist.insert("D".to_string(), now + 3); // out of range

    assert_eq!(skiplist.tick(), vec!["A"]);
    assert_eq!(skiplist.tick(), vec!["B"]);
    assert_eq!(skiplist.tick(), vec!["C"]);
    assert!(skiplist.tick().is_empty());
}

#[test]
fn test_len_simulation() {
    let now = Utc::now().timestamp() as usize;
    let skiplist = EnhancedSkipListExpiry::with_start_time(3, 1, now);

    skiplist.insert("X".to_string(), now);
    skiplist.insert("Y".to_string(), now + 1);
    skiplist.insert("Z".to_string(), now + 2);

    assert_eq!(skiplist.len(), 3);
    skiplist.tick();
    assert_eq!(skiplist.len(), 2);
    skiplist.tick();
    assert_eq!(skiplist.len(), 1);
    skiplist.tick();
    assert_eq!(skiplist.len(), 0);
}

#[test]
fn test_multiple_insert_same_bucket() {
    let now = Utc::now().timestamp() as usize;
    let skiplist = EnhancedSkipListExpiry::with_start_time(2, 1, now);

    skiplist.insert("1".to_string(), now);
    skiplist.insert("2".to_string(), now);
    skiplist.insert("3".to_string(), now);

    let expired = skiplist.tick();
    assert_eq!(expired.len(), 3);
    assert!(expired.contains(&"1".to_string()));
    assert!(expired.contains(&"2".to_string()));
    assert!(expired.contains(&"3".to_string()));
}

#[test]
fn test_out_of_range_insert_ignored() {
    let now = Utc::now().timestamp() as usize;
    let skiplist = EnhancedSkipListExpiry::with_start_time(2, 10, now);

    skiplist.insert("TooEarly".to_string(), now - 100);
    skiplist.insert("TooLate".to_string(), now + 100);

    assert_eq!(skiplist.len(), 0);
}

#[test]
fn test_duplicate_insert_ignored() {
    let now = Utc::now().timestamp() as usize;
    let skiplist = EnhancedSkipListExpiry::with_start_time(3, 1, now);

    skiplist.insert("dup".to_string(), now);
    skiplist.insert("dup".to_string(), now);

    assert_eq!(skiplist.len(), 1);
}

#[test]
fn test_large_volume() {
    let now = Utc::now().timestamp() as usize;
    let skiplist = EnhancedSkipListExpiry::with_start_time(10, 5, now);

    for i in 0..10_000 {
        skiplist.insert(format!("Item-{i}"), now + (i % 50));
    }

    assert_eq!(skiplist.len(), 10_000);
}
