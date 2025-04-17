use crate::time_indexed::skip_list_with_expiry::traits::ConcurrentIndexBasedExpiry;
use std::sync::Arc;
use std::thread;

pub fn threading_test_concurrent_insertions<E>(expiry: Arc<E>)
where
    E: ConcurrentIndexBasedExpiry<String> + 'static,
{
    let mut handles = vec![];

    for i in 0..10 {
        let expiry_clone = Arc::clone(&expiry);
        let id = format!("Item-{}", i);

        let handle = thread::spawn(move || {
            expiry_clone.insert(id, i);
        });

        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    assert_eq!(expiry.len(), 10);
}

pub fn threading_test_concurrent_expiry_behavior<E>(expiry: Arc<E>)
where
    E: ConcurrentIndexBasedExpiry<String> + 'static,
{
    for i in 0..5 {
        expiry.insert(format!("Preload-{}", i), i);
    }

    let mut handles = vec![];

    for _ in 0..3 {
        let expiry_clone = Arc::clone(&expiry);

        let handle = thread::spawn(move || {
            let _ = expiry_clone.expire_front();
        });

        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    assert!(expiry.is_empty());
}

pub fn threading_test_concurrent_mixed_read_write<E>(expiry: Arc<E>)
where
    E: ConcurrentIndexBasedExpiry<String> + 'static,
{
    let mut handles = vec![];

    for i in 0..5 {
        let expiry_clone = Arc::clone(&expiry);
        let id = format!("Item-{}", i);

        let handle = thread::spawn(move || {
            expiry_clone.insert(id, i);
        });

        handles.push(handle);
    }

    for _ in 0..3 {
        let expiry_clone = Arc::clone(&expiry);

        let handle = thread::spawn(move || {
            let _ = expiry_clone.expire_front();
        });

        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    assert!(expiry.len() <= 5);
}
