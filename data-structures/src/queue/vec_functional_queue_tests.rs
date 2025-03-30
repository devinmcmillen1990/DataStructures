use super::vec_functional_queue::VecFunctionalQueue;
use std::thread;

#[test]
fn enqueue_and_dequeue() {
    let q = VecFunctionalQueue::new().enqueue(1).enqueue(2).enqueue(3);
    let (v1, q) = q.dequeue().unwrap();
    let (v2, q) = q.dequeue().unwrap();
    let q = q.enqueue(4);
    let (v3, q) = q.dequeue().unwrap();
    let (v4, _) = q.dequeue().unwrap();

    assert_eq!(v1, 1);
    assert_eq!(v2, 2);
    assert_eq!(v3, 3);
    assert_eq!(v4, 4);
}

#[test]
fn concurrency_safety_simulation() {
    let queue = VecFunctionalQueue::new().enqueue(1).enqueue(2).enqueue(3);

    let handle1 = thread::spawn({
        let q = queue.clone();
        move || {
            let (val, _) = q.dequeue().unwrap();
            assert_eq!(val, 1);
        }
    });

    let handle2 = thread::spawn({
        let q = queue.clone().enqueue(4);
        move || {
            let (val, _) = q.dequeue().unwrap();
            assert_eq!(val, 1);
        }
    });

    handle1.join().unwrap();
    handle2.join().unwrap();
}

#[test]
fn dequeue_from_empty_returns_none() {
    let queue = VecFunctionalQueue::<i32>::new();
    assert!(queue.dequeue().is_none());
}

#[test]
fn enqueue_and_dequeue_one_element() {
    let queue = VecFunctionalQueue::new().enqueue(42);
    let (val, q) = queue.dequeue().unwrap();
    assert_eq!(val, 42);
    assert!(q.is_empty());
}

#[test]
fn persistence_check() {
    let q1 = VecFunctionalQueue::new().enqueue(1).enqueue(2);
    let _q2 = q1.clone().enqueue(3);
    let (val, _) = q1.dequeue().unwrap();

    assert_eq!(val, 1);
}