use std::rc::Rc;

/*
    Uses std::rc to enable structural sharing
*/
#[derive(Clone, Debug)]
pub enum SinglyLinkedList<T> {
    Empty,
    Cons(T, Rc<SinglyLinkedList<T>>),
}

impl<T> SinglyLinkedList<T> {
    pub fn new() -> Self {
        SinglyLinkedList::Empty
    }

    pub fn prepend(self: Rc<Self>, value: T) -> SinglyLinkedList<T> {
        SinglyLinkedList::Cons(value, self)
    }

    pub fn head(&self) -> Option<&T> {
        match self {
            SinglyLinkedList::Cons(value, _) => Some(value),
            _ => None,
        }
    }

    pub fn tail(&self) -> Option<Rc<SinglyLinkedList<T>>> {
        match self {
            SinglyLinkedList::Cons(_, tail) => Some(tail.clone()),
            _ => None,
        }
    }

    pub fn is_empty(&self) -> bool {
        matches!(self, SinglyLinkedList::Empty)
    }

    pub fn reverse(list: &Rc<SinglyLinkedList<T>>) -> Rc<SinglyLinkedList<T>>
    where
        T: Clone,
    {
        let mut result = Rc::new(SinglyLinkedList::Empty);
        let mut current = list.clone();
        while let SinglyLinkedList::Cons(ref head, ref tail) = *current {
            result = Rc::new(SinglyLinkedList::Cons(head.clone(), result));
            current = tail.clone();
        }
        result
    }

    pub fn len(&self) -> usize {
        match self {
            SinglyLinkedList::Empty => 0,
            SinglyLinkedList::Cons(_, tail) => 1 + tail.len(),
        }
    }
}
