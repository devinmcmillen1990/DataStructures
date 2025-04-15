use crate::time_indexed::segmented_time_heap::SegmentedHeap;
use chrono::{Duration, Utc};

#[test]
fn test_insert_and_expire_basic() {
    let heap = SegmentedHeap::new(3, 10);   // 3 buckets, each 10 seconds wide

    let now = Utc::now().timestamp();
    heap.insert("A".to_string(), now);      // bucket 0
    heap.insert("B".to_string(), now + 10); // bucket 1
    heap.insert("C".to_string(), now + 20); // bucket 2
    heap.insert("D".to_string(), now + 30); // too far

    // First expire
    let expired = heap.expire_front();
    assert_eq!(expired, vec!["A"]);

    // Second expire
    let expired = heap.expire_front();
    assert_eq!(expired, vec!["B"]);

    // Third expire
    let expired = heap.expire_front();
    assert_eq!(expired, vec!["C"]);

    // Fourth (wrapped) should be empty
    let expired = heap.expire_front();
    assert!(expired.is_empty());
}

#[test]
fn test_out_of_range_insertion_ignored() {
    let heap = SegmentedHeap::new(2, 10);
    let now = Utc::now().timestamp();

    heap.insert("TooEarly".to_string(), now - 100);
    heap.insert("TooLate".to_string(), now + 100);

    assert_eq!(heap.len(), 0);
}

#[test]
fn test_len_and_is_empty() {
    let heap = SegmentedHeap::new(2, 10);
    let now = Utc::now().timestamp();

    assert!(heap.is_empty());
    heap.insert("X".to_string(), now);
    heap.insert("Y".to_string(), now + 10);

    assert_eq!(heap.len(), 2);

    heap.expire_front(); // "X" expired
    assert_eq!(heap.len(), 1);

    heap.expire_front(); // "Y" expired
    assert!(heap.is_empty());
}

#[test]
fn test_multiple_inserts_same_bucket() {
    let heap = SegmentedHeap::new(2, 10);
    let now = Utc::now().timestamp();

    heap.insert("1".to_string(), now);
    heap.insert("2".to_string(), now);
    heap.insert("3".to_string(), now);

    let expired = heap.expire_front();
    assert_eq!(expired.len(), 3);
    assert!(expired.contains(&"1".to_string()));
    assert!(expired.contains(&"2".to_string()));
    assert!(expired.contains(&"3".to_string()));
}

#[test]
fn test_thread_safe_access() {
    use std::thread;

    let heap = SegmentedHeap::new(3, 10);
    let now = Utc::now().timestamp();
    let h1 = heap.clone();
    let h2 = heap.clone();

    let t1 = thread::spawn(move || {
        h1.insert("T1".to_string(), now);
    });

    let t2 = thread::spawn(move || {
        h2.insert("T2".to_string(), now + 10);
    });

    t1.join().unwrap();
    t2.join().unwrap();

    assert_eq!(heap.len(), 2);
}

#[test]
fn test_large_number_of_inserts() {
    let heap = SegmentedHeap::new(10, 5);
    let now = Utc::now().timestamp();

    for i in 0..10_000 {
        heap.insert(format!("Item-{i}"), now + (i % 50) as i64);
    }

    assert_eq!(heap.len(), 10_000);
}

#[test]
fn test_concurrent_expiry_does_not_panic() {
    use std::sync::Arc;
    use std::thread;

    let heap = Arc::new(SegmentedHeap::new(5, 10));
    let now = Utc::now().timestamp();

    let h_insert = {
        let heap = heap.clone();
        thread::spawn(move || {
            for i in 0..1000 {
                heap.insert(format!("T-{i}"), now + (i % 30));
            }
        })
    };

    let h_expire = {
        let heap = heap.clone();
        thread::spawn(move || {
            for _ in 0..10 {
                let _ = heap.expire_front();
            }
        })
    };

    h_insert.join().unwrap();
    h_expire.join().unwrap();
}
