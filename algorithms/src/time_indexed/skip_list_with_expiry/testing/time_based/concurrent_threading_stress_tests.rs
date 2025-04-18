use crate::time_indexed::skip_list_with_expiry::traits::ConcurrentTimeBasedExpiry;
use chrono::Utc;
use std::sync::Arc;
use std::thread;

pub fn threading_stress_test_concurrent_insertions<E>(expiry: Arc<E>)
where
    E: ConcurrentTimeBasedExpiry<String> + 'static,
{
    let now = Utc::now().timestamp();

    let handles: Vec<_> = (0..100)
        .map(|i| {
            let expiry_ref = Arc::clone(&expiry);
            let id = format!("ID-{}", i);
            thread::spawn(move || {
                expiry_ref.insert(id, now + (i % 10));
            })
        })
        .collect();

    for h in handles {
        h.join().unwrap();
    }

    assert_eq!(expiry.len(), 100);
}

pub fn threading_stress_test_concurrent_expiry_behavior<E>(expiry: Arc<E>)
where
    E: ConcurrentTimeBasedExpiry<String> + 'static,
{
    let now = Utc::now().timestamp();

    for i in 0..100 {
        expiry.insert(format!("Preload-{}", i), now);
    }

    let handles: Vec<_> = (0..20)
        .map(|_| {
            let expiry_ref = Arc::clone(&expiry);
            thread::spawn(move || {
                let _ = expiry_ref.expire_front();
            })
        })
        .collect();

    for h in handles {
        h.join().unwrap();
    }

    assert!(expiry.is_empty());
}

pub fn threading_stress_test_concurrent_mixed_read_write<E>(expiry: Arc<E>)
where
    E: ConcurrentTimeBasedExpiry<String> + 'static,
{
    let now = Utc::now().timestamp();

    // Prime it
    for i in 0..50 {
        expiry.insert(format!("Init-{}", i), now + (i % 3));
    }

    let handles: Vec<_> = (0..200)
        .map(|i| {
            let expiry_ref = Arc::clone(&expiry);
            thread::spawn(move || {
                if i % 2 == 0 {
                    expiry_ref.insert(format!("ID-{}", i), now + (i % 5));
                } else {
                    let _ = expiry_ref.expire_front();
                }
            })
        })
        .collect();

    for h in handles {
        h.join().unwrap();
    }

    assert!(expiry.len() <= 150); // allows for some expired values
}
