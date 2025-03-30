use super::list_functional_queue::ListFunctionalQueue;

#[test]
fn enqueue_and_dequeue() {
    let q = ListFunctionalQueue::new();
    let q = q.enqueue(1).enqueue(2).enqueue(3);
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
fn dequeue_empty_returns_none() {
    let queue = ListFunctionalQueue::<i32>::new();
    assert!(queue.dequeue().is_none());
}

#[test]
fn enqueue_dequeue_single_element() {
    let queue = ListFunctionalQueue::new().enqueue(42);
    let (val, q) = queue.dequeue().unwrap();
    assert_eq!(val, 42);
    assert!(q.is_empty());
}

#[test]
fn persistence_check() {
    let q1 = ListFunctionalQueue::new().enqueue(1).enqueue(2);
    let _q2 = q1.clone().enqueue(3);
    let (val, _) = q1.dequeue().unwrap();

    assert_eq!(val, 1);
}
