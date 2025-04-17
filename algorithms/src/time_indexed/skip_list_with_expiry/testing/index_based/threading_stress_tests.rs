use crate::time_indexed::skip_list_with_expiry::traits::IndexBasedExpiry;
use std::sync::{Arc, Mutex};
use std::thread;

pub fn threading_stress_test_concurrent_insertions<E>(mut expiry: E)
where
    E: IndexBasedExpiry<String> + Send + 'static,
{
    let expiry = Arc::new(Mutex::new(expiry)); // Wrap in Arc<Mutex> for safe concurrent access

    let mut handles = vec![];

    // Stress-test by inserting 100 items concurrently in multiple threads
    for i in 0..100 {
        let expiry_clone = Arc::clone(&expiry);
        let id = format!("StressItem-{}", i);

        let handle = thread::spawn(move || {
            let mut expiry = expiry_clone.lock().unwrap();
            expiry.insert(id, i);
        });
        handles.push(handle);
    }

    // Wait for all threads to finish
    for handle in handles {
        handle.join().unwrap();
    }

    // After all insertions, check if the length is correct
    let expiry = expiry.lock().unwrap();
    assert_eq!(expiry.len(), 100);
}

pub fn threading_stress_test_concurrent_expiry_behavior<E>(mut expiry: E)
where
    E: IndexBasedExpiry<String> + Send + 'static,
{
    let expiry = Arc::new(Mutex::new(expiry)); // Wrap in Arc<Mutex> for safe concurrent access

    // Preload some values
    let now = 100;
    for i in 0..50 {
        let mut expiry = expiry.lock().unwrap();
        expiry.insert(format!("Preload-{}", i), now + i);
    }

    let mut handles = vec![];

    // Stress-test by expiring items concurrently in multiple threads
    for _ in 0..50 {
        let expiry_clone = Arc::clone(&expiry);

        let handle = thread::spawn(move || {
            let mut expiry = expiry_clone.lock().unwrap();
            let expired = expiry.expire_front();
            println!("Expired items: {:?}", expired);
        });
        handles.push(handle);
    }

    // Wait for all threads to finish
    for handle in handles {
        handle.join().unwrap();
    }

    // After expiry, check if the skip list is empty
    let expiry = expiry.lock().unwrap();
    assert!(expiry.is_empty());
}

pub fn threading_stress_test_concurrent_mixed_read_write<E>(mut expiry: E)
where
    E: IndexBasedExpiry<String> + Send + 'static,
{
    let expiry = Arc::new(Mutex::new(expiry)); // Wrap in Arc<Mutex> for safe concurrent access

    let mut handles = vec![];

    // Insert items concurrently in multiple threads
    for i in 0..50 {
        let expiry_clone = Arc::clone(&expiry);
        let id = format!("StressI-{}", i);

        let handle = thread::spawn(move || {
            let mut expiry = expiry_clone.lock().unwrap();
            expiry.insert(id, i);
        });
        handles.push(handle);
    }

    // Expire items concurrently in multiple threads
    for _ in 0..50 {
        let expiry_clone = Arc::clone(&expiry);

        let handle = thread::spawn(move || {
            let mut expiry = expiry_clone.lock().unwrap();
            let _ = expiry.expire_front();
        });
        handles.push(handle);
    }

    // Wait for all threads to finish
    for handle in handles {
        handle.join().unwrap();
    }

    // After all operations, check if the skip list is in a valid state
    let expiry = expiry.lock().unwrap();
    assert!(expiry.len() <= 50); // Ensure no more than 50 items are in the list
}
