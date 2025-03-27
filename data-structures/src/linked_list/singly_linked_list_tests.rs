use super::singly_linked_list::SinglyLinkedList;
use std::rc::Rc;

#[test]
fn test_len() {
    let list = Rc::new(SinglyLinkedList::new());
    assert_eq!(list.len(), 0);

    let list = Rc::new(list.prepend(3));
    let list = Rc::new(list.prepend(2));
    let list = Rc::new(list.prepend(1));

    assert_eq!(list.len(), 3);
}

#[test]
fn test_head_and_tail() {
    let list = Rc::new(SinglyLinkedList::new());
    let list = Rc::new(list.prepend("c"));
    let list = Rc::new(list.prepend("b"));
    let list = Rc::new(list.prepend("a"));

    assert_eq!(list.head(), Some(&"a"));
    let tail = list.tail().unwrap();
    assert_eq!(tail.head(), Some(&"b"));
    let tail = tail.tail().unwrap();
    assert_eq!(tail.head(), Some(&"c"));
    assert!(tail.tail().unwrap().is_empty());
}

#[test]
fn test_reverse() {
    let list = Rc::new(SinglyLinkedList::new());
    let list = Rc::new(list.prepend(3));
    let list = Rc::new(list.prepend(2));
    let list = Rc::new(list.prepend(1));

    let reversed = SinglyLinkedList::reverse(&list);
    assert_eq!(reversed.head(), Some(&3));
    assert_eq!(reversed.tail().unwrap().head(), Some(&2));
    assert_eq!(reversed.tail().unwrap().tail().unwrap().head(), Some(&1));
}
