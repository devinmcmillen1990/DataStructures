use crate::time_indexed_enhanced2::skip_list_expiry_enhanced2::EnhancedSkipListExpiry2;
use chrono::Utc;

#[test]
fn test_insert_with_expiry_ticks() {
    let skiplist = EnhancedSkipListExpiry2::with_start_time(3, 1, 0);

    skiplist.insert("A", 0);
    skiplist.insert("B", 1);
    skiplist.insert("C", 2);
    skiplist.insert("D", 3);

    assert_eq!(skiplist.tick(), vec!["A"]);
    assert_eq!(skiplist.tick(), vec!["B"]);
    assert_eq!(skiplist.tick(), vec!["C"]);
    assert_eq!(skiplist.tick(), Vec::<&str>::new());
}

#[test]
fn test_out_of_range_insert_ignored() {
    let skiplist = EnhancedSkipListExpiry2::with_start_time(3, 1, 100);

    skiplist.insert("TooEarly", 99); // Too early, should be ignored
    skiplist.insert("TooLate", 200); // Too far in future, also ignored

    assert_eq!(skiplist.len(), 0);
}

#[test]
fn test_len_simulation() {
    let skiplist = EnhancedSkipListExpiry2::with_start_time(3, 1, 0);
    skiplist.insert("X", 0);
    skiplist.insert("Y", 1);
    skiplist.insert("Z", 2);
    assert_eq!(skiplist.len(), 3);

    skiplist.tick();
    assert_eq!(skiplist.len(), 2);

    skiplist.tick();
    assert_eq!(skiplist.len(), 1);

    skiplist.tick();
    assert_eq!(skiplist.len(), 0);
}

#[test]
fn test_duplicate_insert_ignored() {
    let skiplist = EnhancedSkipListExpiry2::with_start_time(3, 1, 0);
    skiplist.insert("DUPLICATE", 1);
    skiplist.insert("DUPLICATE", 1);
    assert_eq!(skiplist.len(), 1);
}

#[test]
fn test_multiple_insert_same_bucket() {
    let skiplist = EnhancedSkipListExpiry2::with_start_time(3, 1, 0);
    skiplist.insert("1", 1);
    skiplist.insert("2", 1);
    skiplist.insert("3", 1);

    skiplist.tick(); // Advance to slot 1
    let expired = skiplist.tick(); // Should return all 3
    assert_eq!(expired.len(), 3);
    assert!(expired.contains(&"1"));
    assert!(expired.contains(&"2"));
    assert!(expired.contains(&"3"));
}

#[test]
fn test_large_volume() {
    let skiplist = EnhancedSkipListExpiry2::with_start_time(200, 1, 0);
    for i in 0..10_000 {
        skiplist.insert(format!("item-{i}"), (i % 50) + 1);
    }
    assert_eq!(skiplist.len(), 10_000);
}
