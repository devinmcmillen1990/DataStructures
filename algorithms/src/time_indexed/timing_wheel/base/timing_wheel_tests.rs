use crate::time_indexed::timing_wheel::TimingWheel;

// TODO: Re-evaluate these at a later point because there were a ton of changes.

#[test]
fn test_insert_and_tick_basic() {
    let mut wheel = TimingWheel::new(4, 10);
    let base = wheel.start_time();

    wheel.insert("A".to_string(), base + 10); // slot 1
    wheel.insert("B".to_string(), base + 20); // slot 2
    wheel.insert("C".to_string(), base + 30); // slot 3
    wheel.insert("D".to_string(), base + 0); // slot 0

    // Tick from slot 0 to 3
    assert_eq!(wheel.tick(), vec!["D"]); // slot 0
    assert_eq!(wheel.tick(), vec!["A"]); // slot 1
    assert_eq!(wheel.tick(), vec!["B"]); // slot 2
    assert_eq!(wheel.tick(), vec!["C"]); // slot 3
    assert!(wheel.tick().is_empty()); // back to slot 0 again (now empty)
}

#[test]
fn test_len_and_is_empty() {
    let mut wheel = TimingWheel::new(3, 10);
    let base = wheel.start_time();

    assert!(wheel.is_empty());

    wheel.insert("X".to_string(), base + 10); // slot 1
    wheel.insert("Y".to_string(), base + 20); // slot 2
    assert_eq!(wheel.len(), 2);

    wheel.tick(); // slot 0 → empty
    assert_eq!(wheel.len(), 2); // still full

    wheel.tick(); // slot 1 → X expired
    assert_eq!(wheel.len(), 1);

    wheel.tick(); // slot 2 → Y expired
    assert!(wheel.is_empty());
}

#[test]
fn test_multiple_inserts_same_slot() {
    let mut wheel = TimingWheel::new(2, 10);
    let base = wheel.start_time();

    wheel.insert("1".to_string(), base + 10); // slot 1
    wheel.insert("2".to_string(), base + 10); // slot 1
    wheel.insert("3".to_string(), base + 10); // slot 1

    wheel.tick(); // slot 0 → empty
    let expired = wheel.tick(); // slot 1 → all 3 should expire

    assert_eq!(expired.len(), 3);
    assert!(expired.contains(&"1".to_string()));
    assert!(expired.contains(&"2".to_string()));
    assert!(expired.contains(&"3".to_string()));
}
