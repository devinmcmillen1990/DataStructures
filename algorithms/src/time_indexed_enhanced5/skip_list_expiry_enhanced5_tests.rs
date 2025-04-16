use crate::time_indexed_enhanced5::skip_list_expiry_enhanced5::EnhancedSkipListExpiry5;
use std::collections::HashSet;
use std::sync::{Arc, RwLock};
use std::thread;

#[test]
fn test_basic_expiry() {
    let mut skiplist = EnhancedSkipListExpiry5::new(4);
    skiplist.insert("a".to_string(), 4);
    skiplist.insert("b".to_string(), 4);
    let expired = skiplist.tick(4);
    assert_eq!(expired, vec!["a", "b"]);
}

#[test]
fn test_ordered_expiry_over_time() {
    let mut skiplist = EnhancedSkipListExpiry5::new(4);
    skiplist.insert("a".to_string(), 4);
    skiplist.insert("b".to_string(), 8);

    let expired = skiplist.tick(4);
    assert_eq!(expired, vec!["a"]);

    let expired = skiplist.tick(8);
    assert_eq!(expired, vec!["b"]);
}

#[test]
fn test_multiple_insert_same_time() {
    let mut skiplist = EnhancedSkipListExpiry5::new(4);
    skiplist.insert("m1".to_string(), 4);
    skiplist.insert("m2".to_string(), 4);
    skiplist.insert("m3".to_string(), 4);
    let mut expired = skiplist.tick(4);
    expired.sort();
    assert_eq!(expired, vec!["m1", "m2", "m3"]);
}

#[test]
fn test_concurrent_insertions_and_expiry() {
    let skiplist = Arc::new(RwLock::new(EnhancedSkipListExpiry5::new(4)));

    for _ in 0..20 {
        let skiplist_clone = Arc::clone(&skiplist);
        let handles: Vec<_> = (0..4)
            .map(|t| {
                let inner = Arc::clone(&skiplist_clone);
                thread::spawn(move || {
                    for j in 0..10 {
                        let id = format!("t{}-{}", t, j);
                        inner.write().unwrap().insert(id, 4);
                    }
                })
            })
            .collect();

        for h in handles {
            h.join().unwrap();
        }

        let expired = skiplist.write().unwrap().tick(4);
        assert_eq!(expired.len(), 40);
    }
}
