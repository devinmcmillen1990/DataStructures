use crate::self_expiring::skip_list_with_expiry::traits::ConcurrentTimeBasedExpiry;
use chrono::Utc;
use std::sync::Arc;
use std::thread;

pub fn threading_test_concurrent_insertions<E>(expiry: Arc<E>)
where
    E: ConcurrentTimeBasedExpiry<String> + 'static,
{
    let now = Utc::now().timestamp();

    let handles: Vec<_> = (0..10)
        .map(|i| {
            let expiry_ref = Arc::clone(&expiry);
            let id = format!("Thread-{}", i);
            thread::spawn(move || {
                expiry_ref.insert(id, now + (i % 5));
            })
        })
        .collect();

    for handle in handles {
        handle.join().unwrap();
    }

    assert_eq!(expiry.len(), 10);
}

pub fn threading_test_concurrent_expiry_behavior<E>(expiry: Arc<E>)
where
    E: ConcurrentTimeBasedExpiry<String> + 'static,
{
    let now = Utc::now().timestamp();

    for i in 0..10 {
        expiry.insert(format!("Preload-{}", i), now);
    }

    let handles: Vec<_> = (0..4)
        .map(|_| {
            let expiry_ref = Arc::clone(&expiry);
            thread::spawn(move || {
                let _ = expiry_ref.expire_front();
            })
        })
        .collect();

    for handle in handles {
        handle.join().unwrap();
    }

    assert!(expiry.is_empty());
}

pub fn threading_test_concurrent_mixed_read_write<E>(expiry: Arc<E>)
where
    E: ConcurrentTimeBasedExpiry<String> + 'static,
{
    let now = Utc::now().timestamp();

    // Prime with some values
    for i in 0..5 {
        expiry.insert(format!("Init-{}", i), now + 1);
    }

    let handles: Vec<_> = (0..20)
        .map(|i| {
            let expiry_ref = Arc::clone(&expiry);
            thread::spawn(move || {
                if i % 3 == 0 {
                    let _ = expiry_ref.expire_front();
                } else {
                    expiry_ref.insert(format!("ID-{}", i), now + (i % 5));
                }
            })
        })
        .collect();

    for handle in handles {
        handle.join().unwrap();
    }

    let values = expiry.values();
    assert!(
        values.len() <= 25,
        "Too many items remain after concurrent mixed activity"
    );
}
