use super::super::time_indexed_enhanced4::skip_list_expiry_enhanced4::EnhancedSkipListExpiry4;

#[test]
fn test_insert_with_expiry_ticks() {
    let skiplist = EnhancedSkipListExpiry4::new();
    let base = 1000;

    skiplist.insert("A".to_string(), base + 1);
    skiplist.insert("B".to_string(), base + 2);
    skiplist.insert("C".to_string(), base + 3);

    assert_eq!(skiplist.tick(base + 0), Vec::<String>::new());
    assert_eq!(skiplist.tick(base + 1), vec!["A".to_string()]);
    assert_eq!(skiplist.tick(base + 2), vec!["B".to_string()]);
    assert_eq!(skiplist.tick(base + 3), vec!["C".to_string()]);
    assert_eq!(skiplist.tick(base + 4), Vec::<String>::new());
}

#[test]
fn test_len_tracking() {
    let skiplist = EnhancedSkipListExpiry4::new();
    let base = 1000;

    skiplist.insert("A".to_string(), base + 1);
    skiplist.insert("B".to_string(), base + 2);
    skiplist.insert("C".to_string(), base + 3);
    assert_eq!(skiplist.len(), 3);

    skiplist.tick(base + 1);
    let expired = skiplist.tick(base + 2);
    assert_eq!(expired.len(), 1);
    assert_eq!(skiplist.len(), 1);
}

#[test]
fn test_multiple_insert_same_bucket() {
    let skiplist = EnhancedSkipListExpiry4::new();
    let base = 1000;
    let expire_at = base + 2;

    skiplist.insert("A", expire_at);
    skiplist.insert("B", expire_at);
    skiplist.insert("C", expire_at);

    // Tick up to (but not including) expire time
    skiplist.tick(base);
    skiplist.tick(base + 1);
    assert_eq!(skiplist.len(), 3);

    // This tick should expire all 3
    let expired = skiplist.tick(expire_at);
    let mut expired_sorted = expired;
    expired_sorted.sort();
    assert_eq!(expired_sorted, vec!["A", "B", "C"]);
    assert_eq!(skiplist.len(), 0);
}

#[test]
fn test_large_volume() {
    let skiplist = EnhancedSkipListExpiry4::new();
    let base = 1000;
    let count = 10_000;
    for i in 0..count {
        skiplist.insert(format!("Item-{}", i), base + (i % 100));
    }

    let mut expired_total = 0;
    for i in 0..100 {
        expired_total += skiplist.tick(base + i).len();
    }

    assert_eq!(expired_total, count);
}
