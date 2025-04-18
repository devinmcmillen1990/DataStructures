use crate::time_indexed::skip_list_with_expiry::traits::ConcurrentIndexBasedExpiry;
use std::sync::Arc;
use std::thread;

pub fn threading_test_concurrent_insertions<E>(expiry: Arc<E>)
where
    E: ConcurrentIndexBasedExpiry<String> + 'static,
{
    let handles: Vec<_> = (0..10)
        .map(|i| {
            let expiry_ref = Arc::clone(&expiry);
            let id = format!("ID-{}", i);
            thread::spawn(move || {
                expiry_ref.insert(id, i % 3);
            })
        })
        .collect();

    for h in handles {
        h.join().unwrap();
    }

    assert_eq!(expiry.len(), 10);
}

pub fn threading_test_concurrent_expiry_behavior<E>(expiry: Arc<E>)
where
    E: ConcurrentIndexBasedExpiry<String> + 'static,
{
    let bucket_distribution_factor = 4;
    
    for i in 0..10 {
        expiry.insert(format!("Preload-{}", i), i % bucket_distribution_factor);
    }

    let handles: Vec<_> = (0..bucket_distribution_factor)
        .map(|_| {
            let expiry_ref = Arc::clone(&expiry);
            thread::spawn(move || {
                let expired = expiry_ref.expire_front();
                eprint!("\nExpired={:?}\n", expired);
            })
        })
        .collect();

    for h in handles {
        h.join().unwrap();
    }

    eprint!("\nValues = {:?}\n", expiry.values());

    assert!(expiry.is_empty());
}

pub fn threading_test_concurrent_mixed_read_write<E>(expiry: Arc<E>)
where
    E: ConcurrentIndexBasedExpiry<String> + 'static,
{
    for i in 0..50 {
        expiry.insert(format!("Initial-{}", i), i % 3);
    }

    let handles: Vec<_> = (0..100)
        .map(|i| {
            let expiry_ref = Arc::clone(&expiry);
            thread::spawn(move || {
                if i % 2 == 0 {
                    expiry_ref.insert(format!("ID-{}", i), i % 3);
                } else {
                    let _ = expiry_ref.expire_front();
                }
            })
        })
        .collect();

    for h in handles {
        h.join().unwrap();
    }

    assert!(expiry.len() <= 50);
}
