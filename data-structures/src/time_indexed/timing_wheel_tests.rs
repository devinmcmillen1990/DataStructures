use crate::time_indexed::timing_wheel::TimingWheel;
use chrono::Utc;

#[test]
fn test_insert_and_tick_basic() {
    let wheel = TimingWheel::new(4, 10); // 4 slots, 10s resolution
    let now = Utc::now().timestamp();

    wheel.insert("A".to_string(), now + 10); // slot 1
    wheel.insert("B".to_string(), now + 20); // slot 2
    wheel.insert("C".to_string(), now + 30); // slot 3
    wheel.insert("D".to_string(), now + 40); // wraps to slot 0

    assert_eq!(wheel.tick(), vec!["A"]);
    assert_eq!(wheel.tick(), vec!["B"]);
    assert_eq!(wheel.tick(), vec!["C"]);
    assert_eq!(wheel.tick(), vec!["D"]);
    assert!(wheel.tick().is_empty());
}

#[test]
fn test_out_of_range_is_ignored() {
    let wheel = TimingWheel::new(3, 5);
    let now = Utc::now().timestamp();

    wheel.insert("Expired".to_string(), now - 30); // already expired

    assert!(wheel.is_empty());
}

#[test]
fn test_len_and_is_empty() {
    let wheel = TimingWheel::new(3, 10);
    let now = Utc::now().timestamp();

    assert!(wheel.is_empty());

    wheel.insert("X".to_string(), now + 10);
    wheel.insert("Y".to_string(), now + 20);

    assert_eq!(wheel.len(), 2);

    wheel.tick(); // "X"
    assert_eq!(wheel.len(), 1);

    wheel.tick(); // "Y"
    assert!(wheel.is_empty());
}

#[test]
fn test_multiple_inserts_same_slot() {
    let wheel = TimingWheel::new(2, 10);
    let now = Utc::now().timestamp();

    wheel.insert("1".to_string(), now + 10);
    wheel.insert("2".to_string(), now + 10);
    wheel.insert("3".to_string(), now + 10);

    let expired = wheel.tick(); // should hit slot 1
    assert_eq!(expired.len(), 3);
    assert!(expired.contains(&"1".to_string()));
    assert!(expired.contains(&"2".to_string()));
    assert!(expired.contains(&"3".to_string()));
}

#[test]
fn test_thread_safe_access() {
    use std::thread;

    let wheel = TimingWheel::new(4, 5);
    let now = Utc::now().timestamp();

    let w1 = wheel.clone();
    let w2 = wheel.clone();

    let t1 = thread::spawn(move || {
        w1.insert("T1".to_string(), now + 5);
    });

    let t2 = thread::spawn(move || {
        w2.insert("T2".to_string(), now + 10);
    });

    t1.join().unwrap();
    t2.join().unwrap();

    assert_eq!(wheel.len(), 2);
}

#[test]
fn test_large_number_of_inserts() {
    let wheel = TimingWheel::new(20, 10);
    let now = Utc::now().timestamp();

    for i in 0..10_000 {
        wheel.insert(format!("Item-{i}"), now + ((i % 180) as i64));
    }

    assert_eq!(wheel.len(), 10_000);
}

#[test]
fn test_tick_concurrent_does_not_panic() {
    use std::sync::Arc;
    use std::thread;

    let wheel = Arc::new(TimingWheel::new(10, 10));
    let now = Utc::now().timestamp();

    let h_insert = {
        let wheel = wheel.clone();
        thread::spawn(move || {
            for i in 0..1000 {
                wheel.insert(format!("X-{i}"), now + (i % 30));
            }
        })
    };

    let h_expire = {
        let wheel = wheel.clone();
        thread::spawn(move || {
            for _ in 0..10 {
                let _ = wheel.tick();
            }
        })
    };

    h_insert.join().unwrap();
    h_expire.join().unwrap();
}
