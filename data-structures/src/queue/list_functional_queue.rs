use std::rc::Rc;
use crate::linked_list::singly_linked_list::SinglyLinkedList;

#[derive(Clone, Debug)]
pub struct ListFunctionalQueue<T> {
    front:  Rc<SinglyLinkedList<T>>,
    rear:   Rc<SinglyLinkedList<T>>,
}

impl<T: Clone> ListFunctionalQueue<T> {
    pub fn new() -> Self {
        Self {
            front:  Rc::new(SinglyLinkedList::Empty),
            rear:   Rc::new(SinglyLinkedList::Empty),
        }
    }

    pub fn is_empty(&self) -> bool {
        self.front.is_empty() && self.rear.is_empty()
    }

    pub fn enqueue(&self, item: T) -> Self {
        let new_rear = Rc::new(self.rear.clone().prepend(item));
        self.check(self.front.clone(), new_rear)
    }

    pub fn dequeue(&self) -> Option<(T, Self)> {
        match &*self.front {
            SinglyLinkedList::Cons(head, tail) => {
                let new_q = Self {
                    front: tail.clone(),
                    rear: self.rear.clone(),
                };
                Some((head.clone(), new_q.check(new_q.front.clone(), new_q.rear.clone())))
            }
            SinglyLinkedList::Empty => None,
        }
    }    

    fn check(&self, front: Rc<SinglyLinkedList<T>>, rear: Rc<SinglyLinkedList<T>>) -> Self {
        if front.is_empty() {
            Self {
                front: SinglyLinkedList::reverse(&rear),
                rear: Rc::new(SinglyLinkedList::Empty),
            }
        } else {
            Self { front, rear }
        }
    }    
}
