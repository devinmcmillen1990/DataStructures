use crate::time_indexed::skip_list_with_expiry::traits::ConcurrentIndexBasedExpiry;
use std::sync::Arc;
use std::thread;

pub fn threading_stress_test_concurrent_insertions<E>(expiry: Arc<E>)
where
    E: ConcurrentIndexBasedExpiry<String> + 'static,
{
    let mut handles = vec![];

    for i in 0..100 {
        let expiry_ref = Arc::clone(&expiry);
        let id = format!("StressItem-{}", i);

        let handle = thread::spawn(move || {
            expiry_ref.insert(id, i % 10);
        });

        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    assert_eq!(expiry.len(), 100);
}

pub fn threading_stress_test_concurrent_expiry_behavior<E>(expiry: Arc<E>)
where
    E: ConcurrentIndexBasedExpiry<String> + 'static,
{
    // Preload values
    for i in 0..50 {
        expiry.insert(format!("Preload-{}", i), i % 5);
    }

    let mut handles = vec![];

    for _ in 0..50 {
        let expiry_ref = Arc::clone(&expiry);
        let handle = thread::spawn(move || {
            let _ = expiry_ref.expire_front();
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    assert!(expiry.is_empty());
}

pub fn threading_stress_test_concurrent_mixed_read_write<E>(expiry: Arc<E>)
where
    E: ConcurrentIndexBasedExpiry<String> + 'static,
{
    let mut handles = vec![];

    // Mixed insertions
    for i in 0..50 {
        let expiry_ref = Arc::clone(&expiry);
        let id = format!("StressI-{}", i);

        let handle = thread::spawn(move || {
            expiry_ref.insert(id, i % 5);
        });

        handles.push(handle);
    }

    // Mixed expirations
    for _ in 0..50 {
        let expiry_ref = Arc::clone(&expiry);
        let handle = thread::spawn(move || {
            let _ = expiry_ref.expire_front();
        });

        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    assert!(expiry.len() <= 50);
}
