use crate::time_indexed::skip_list_expiry::SkipListExpiry;
use chrono::Utc;

#[test]
fn test_insert_with_expiry_ticker() {
    // Create a skip list with 3 buckets of 1 second each
    let skiplist = SkipListExpiry::new(3, 1);
    let now = Utc::now().timestamp();

    skiplist.insert("A".to_string(), now);       // Will expire in 1st tick
    skiplist.insert("B".to_string(), now + 1);   // 2nd tick
    skiplist.insert("C".to_string(), now + 2);   // 3rd tick
    skiplist.insert("D".to_string(), now + 3);   // Out of range, ignored

    // Simulate time passing by calling expire_front per tick
    assert_eq!(skiplist.expire_front(), vec!["A"]);
    assert_eq!(skiplist.expire_front(), vec!["B"]);
    assert_eq!(skiplist.expire_front(), vec!["C"]);
    assert!(skiplist.expire_front().is_empty());
}

#[test]
fn test_live_ticker_simulation() {
    // This simulates a runtime scenario where tick is called once per second
    let skiplist = SkipListExpiry::new(5, 1);
    let now = Utc::now().timestamp();

    skiplist.insert("First".to_string(), now);
    skiplist.insert("Second".to_string(), now + 1);
    skiplist.insert("Third".to_string(), now + 2);

    // First tick
    assert_eq!(skiplist.expire_front(), vec!["First"]);

    // Second tick
    assert_eq!(skiplist.expire_front(), vec!["Second"]);

    // Third tick
    assert_eq!(skiplist.expire_front(), vec!["Third"]);
}

#[test]
fn test_len_after_expiry() {
    let skiplist = SkipListExpiry::new(3, 2);
    let now = Utc::now().timestamp();

    skiplist.insert("X".to_string(), now);
    skiplist.insert("Y".to_string(), now + 2);

    assert_eq!(skiplist.len(), 2);

    skiplist.expire_front(); // "X" should expire
    assert_eq!(skiplist.len(), 1);

    skiplist.expire_front(); // "Y" should expire
    assert_eq!(skiplist.len(), 0);
}

#[test]
fn test_insert_and_expire_basic() {
    let skiplist = SkipListExpiry::new(3, 10); // 3 buckets, 10s each

    let now = Utc::now().timestamp();
    skiplist.insert("A".to_string(), now); // bucket 0
    skiplist.insert("B".to_string(), now + 10); // bucket 1
    skiplist.insert("C".to_string(), now + 20); // bucket 2
    skiplist.insert("D".to_string(), now + 30); // too far

    assert_eq!(skiplist.expire_front(), vec!["A"]);
    assert_eq!(skiplist.expire_front(), vec!["B"]);
    assert_eq!(skiplist.expire_front(), vec!["C"]);
    assert!(skiplist.expire_front().is_empty());
}

#[test]
fn test_out_of_range_insertion_ignored() {
    let skiplist = SkipListExpiry::new(2, 10);
    let now = Utc::now().timestamp();

    skiplist.insert("TooEarly".to_string(), now - 100);
    skiplist.insert("TooLate".to_string(), now + 100);

    assert_eq!(skiplist.len(), 0);
}

#[test]
fn test_len_and_is_empty() {
    let skiplist = SkipListExpiry::new(2, 10);
    let now = Utc::now().timestamp();

    assert!(skiplist.is_empty());
    skiplist.insert("X".to_string(), now);
    skiplist.insert("Y".to_string(), now + 10);

    assert_eq!(skiplist.len(), 2);

    skiplist.expire_front(); // removes X
    assert_eq!(skiplist.len(), 1);

    skiplist.expire_front(); // removes Y
    assert!(skiplist.is_empty());
}

#[test]
fn test_multiple_inserts_same_bucket() {
    let skiplist = SkipListExpiry::new(2, 10);
    let now = Utc::now().timestamp();

    skiplist.insert("1".to_string(), now);
    skiplist.insert("2".to_string(), now);
    skiplist.insert("3".to_string(), now);

    let expired = skiplist.expire_front();
    assert_eq!(expired.len(), 3);
    assert!(expired.contains(&"1".to_string()));
    assert!(expired.contains(&"2".to_string()));
    assert!(expired.contains(&"3".to_string()));
}

#[test]
fn test_thread_safe_access() {
    use std::thread;

    let skiplist = SkipListExpiry::new(3, 10);
    let now = Utc::now().timestamp();
    let s1 = skiplist.clone();
    let s2 = skiplist.clone();

    let t1 = thread::spawn(move || {
        s1.insert("T1".to_string(), now);
    });

    let t2 = thread::spawn(move || {
        s2.insert("T2".to_string(), now + 10);
    });

    t1.join().unwrap();
    t2.join().unwrap();

    assert_eq!(skiplist.len(), 2);
}

#[test]
fn test_large_number_of_inserts() {
    let skiplist = SkipListExpiry::new(10, 5);
    let now = Utc::now().timestamp();

    for i in 0..10_000 {
        skiplist.insert(format!("Item-{i}"), now + (i % 50) as i64);
    }

    assert_eq!(skiplist.len(), 10_000);
}

#[test]
fn test_concurrent_expiry_does_not_panic() {
    use std::sync::Arc;
    use std::thread;

    let skiplist = Arc::new(SkipListExpiry::new(5, 10));
    let now = Utc::now().timestamp();

    let h_insert = {
        let skiplist = skiplist.clone();
        thread::spawn(move || {
            for i in 0..1000 {
                skiplist.insert(format!("T-{i}"), now + (i % 30));
            }
        })
    };

    let h_expire = {
        let skiplist = skiplist.clone();
        thread::spawn(move || {
            for _ in 0..10 {
                let _ = skiplist.expire_front();
            }
        })
    };

    h_insert.join().unwrap();
    h_expire.join().unwrap();
}
