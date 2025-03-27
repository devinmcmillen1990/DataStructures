use super::list_functional_queue::ListFunctionalQueue;

#[test]
fn queue_works() {
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
