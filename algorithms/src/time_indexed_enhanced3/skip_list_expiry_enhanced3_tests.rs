use crate::time_indexed_enhanced3::skip_list_expiry_enhanced3::EnhancedSkipListExpiry3;
use chrono::Utc;

#[test]
fn test_insert_with_expiry_ticks() {
    let now = Utc::now().timestamp() as usize;
    let skiplist = EnhancedSkipListExpiry3::with_start_time(3, 1, now);

    skiplist.insert("A", now);
    skiplist.insert("B", now + 1);
    skiplist.insert("C", now + 2);
    skiplist.insert("D", now + 3); // too far

    assert_eq!(skiplist.tick(), vec!["A"]);
    assert_eq!(skiplist.tick(), vec!["B"]);
    assert_eq!(skiplist.tick(), vec!["C"]);
    assert!(skiplist.tick().is_empty());
}

#[test]
fn test_len_tracking() {
    let now = Utc::now().timestamp() as usize;
    let skiplist = EnhancedSkipListExpiry3::with_start_time(3, 1, now);

    assert_eq!(skiplist.len(), 0);
    skiplist.insert("X", now);
    skiplist.insert("Y", now + 1);
    assert_eq!(skiplist.len(), 2);
    skiplist.tick();
    assert_eq!(skiplist.len(), 1);
    skiplist.tick();
    assert_eq!(skiplist.len(), 0);
}

#[test]
fn test_multiple_insert_same_bucket() {
    let now = Utc::now().timestamp() as usize;
    let skiplist = EnhancedSkipListExpiry3::with_start_time(2, 10, now);

    skiplist.insert("1", now);
    skiplist.insert("2", now);
    skiplist.insert("3", now);

    let expired = skiplist.tick();
    assert_eq!(expired.len(), 3);
    assert!(expired.contains(&"1"));
    assert!(expired.contains(&"2"));
    assert!(expired.contains(&"3"));
}

#[test]
fn test_large_volume() {
    let now = Utc::now().timestamp() as usize;
    let skiplist = EnhancedSkipListExpiry3::with_start_time(10, 5, now);

    for i in 0..10_000 {
        skiplist.insert(format!("Item-{i}"), now + (i % 50));
    }

    assert_eq!(skiplist.len(), 10_000);
}
