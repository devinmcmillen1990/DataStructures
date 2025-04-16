use crate::time_indexed::skip_list_with_expiry::base::skip_list_expiry::SkipListExpiry;
use chrono::Utc;
use std::panic::{self, UnwindSafe};
use std::sync::Arc;
use std::thread;
use std::time::Duration;

/// Helper to catch and log expected panics in non-thread-safe structure
fn try_test<F>(label: &str, f: F)
where
    F: FnOnce() + Send + 'static + UnwindSafe,
{
    let result = panic::catch_unwind(f);
    if result.is_err() {
        eprintln!("⚠️ [Expected] {} triggered a race condition panic.", label);
    }
}

/// Spawn threads inserting items concurrently
#[test]
fn test_concurrent_inserts_expected_to_race() {
    let skiplist = Arc::new(SkipListExpiry::new(4, 1));
    let now = Utc::now().timestamp();

    let mut handles = vec![];
    for i in 0..10 {
        let sl = Arc::clone(&skiplist);
        let id = format!("Item-{}", i);
        let now = now;
        handles.push(thread::spawn(move || {
            try_test("insert", move || {
                sl.insert(id.clone(), now + (i % 4));
            });
        }));
    }

    for h in handles {
        let _ = h.join();
    }

    let skiplist = Arc::clone(&skiplist);
    try_test("final values read", move || {
        let values = skiplist.values();
        println!("Snapshot: {:?}", values);
    });
}

/// Spawn threads expiring buckets concurrently
#[test]
fn test_concurrent_expiry_expected_to_race() {
    let skiplist = Arc::new(SkipListExpiry::new(3, 2));
    let now = Utc::now().timestamp();

    for i in 0..3 {
        skiplist.insert(format!("Preload-{}", i), now + (i * 2));
    }

    let mut handles = vec![];
    for _ in 0..3 {
        let sl = Arc::clone(&skiplist);
        handles.push(thread::spawn(move || {
            try_test("expire_front", move || {
                let expired = sl.expire_front();
                println!("Expired items: {:?}", expired);
            });
        }));
    }

    for h in handles {
        let _ = h.join();
    }
}

/// Insert and expire in parallel
#[test]
fn test_concurrent_insert_and_expire_expected_to_race() {
    let skiplist = Arc::new(SkipListExpiry::new(5, 1));
    let now = Utc::now().timestamp();

    let sl_insert = Arc::clone(&skiplist);
    let sl_expire = Arc::clone(&skiplist);

    let inserter = thread::spawn(move || {
        for i in 0..10 {
            let id = format!("I-{}", i);
            let now = now;
            let sl = Arc::clone(&sl_insert); // 💡 fix: clone per iteration

            try_test("insert", move || {
                sl.insert(id, now + (i % 5));
            });

            thread::sleep(Duration::from_millis(10));
        }
    });

    let expirer = thread::spawn(move || {
        for _ in 0..10 {
            let sl = Arc::clone(&sl_expire); // 💡 fix: clone per iteration

            try_test("expire_front", move || {
                let _ = sl.expire_front();
            });

            thread::sleep(Duration::from_millis(10));
        }
    });

    let _ = inserter.join();
    let _ = expirer.join();

    let skiplist = Arc::clone(&skiplist);
    try_test("final snapshot", move || {
        println!("Remaining values: {:?}", skiplist.values());
    });
}

#[test]
#[ignore = "stress-test"]
fn stress_concurrent_inserts_expected_to_race() {
    let skiplist = Arc::new(SkipListExpiry::new(8, 1));
    let now = Utc::now().timestamp();

    let mut handles = vec![];
    for i in 0..100 {
        let sl = Arc::clone(&skiplist);
        let id = format!("StressItem-{}", i);
        let now = now;
        handles.push(thread::spawn(move || {
            try_test("stress:insert", move || {
                sl.insert(id, now + (i % 8));
            });
        }));
    }

    for h in handles {
        let _ = h.join();
    }

    let snapshot = Arc::clone(&skiplist);
    try_test("stress:final-read", move || {
        let values = snapshot.values();
        println!("Stress snapshot: {} values", values.len());
    });
}

#[test]
#[ignore = "stress-test"]
fn stress_concurrent_expiry_expected_to_race() {
    let skiplist = Arc::new(SkipListExpiry::new(5, 1));
    let now = Utc::now().timestamp();
    for i in 0..100 {
        skiplist.insert(format!("StressPreload-{}", i), now + (i % 5));
    }

    let mut handles = vec![];
    for _ in 0..100 {
        let sl = Arc::clone(&skiplist);
        handles.push(thread::spawn(move || {
            try_test("stress:expire", move || {
                let _ = sl.expire_front();
            });
        }));
    }

    for h in handles {
        let _ = h.join();
    }
}

#[test]
#[ignore = "stress-test"]
fn stress_concurrent_insert_and_expire_expected_to_race() {
    let skiplist = Arc::new(SkipListExpiry::new(10, 1));
    let now = Utc::now().timestamp();

    let sl_insert = Arc::clone(&skiplist);
    let sl_expire = Arc::clone(&skiplist);

    let inserter = thread::spawn(move || {
        for i in 0..500 {
            let id = format!("StressI-{}", i);
            let now = now;
            let sl = Arc::clone(&sl_insert);
            try_test("stress:insert", move || {
                sl.insert(id, now + (i % 10));
            });
        }
    });

    let expirer = thread::spawn(move || {
        for _ in 0..500 {
            let sl = Arc::clone(&sl_expire);
            try_test("stress:expire", move || {
                let _ = sl.expire_front();
            });
        }
    });

    let _ = inserter.join();
    let _ = expirer.join();

    let snapshot = Arc::clone(&skiplist);
    try_test("stress:final-values", move || {
        println!("Remaining after stress: {:?}", snapshot.values().len());
    });
}
