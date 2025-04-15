use crate::queue::vec_functional_queue_visual::QueueState;
use data_structures::queue::list_functional_queue::ListFunctionalQueue;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsValue;

/*
    TODO:
        1. Setup SEO to make this searchable (Maybe JS)
        2. Replace custom CSS with TailWindCSS (Maybe JS)
        3. Ensure is mobile friendly (Maybe JS)
        4. Add "Buy me a Coffee" (Maybe JS)
        5. Add Advertisements (Maybe JS)
*/

#[wasm_bindgen]
pub struct ListFunctionalQueueVisual {
    inner: ListFunctionalQueue<String>,
}

#[wasm_bindgen]
impl ListFunctionalQueueVisual {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Self {
        Self {
            inner: ListFunctionalQueue::new(),
        }
    }

    pub fn enqueue(&mut self, value: String) {
        self.inner = self.inner.enqueue(value);
    }

    pub fn dequeue(&mut self) -> JsValue {
        match self.inner.dequeue() {
            Some((val, next)) => {
                self.inner = next;
                JsValue::from_serde(&val).unwrap()
            }
            None => JsValue::NULL,
        }
    }

    pub fn state(&self) -> JsValue {
        let state = QueueState {
            front: flatten_list(self.inner.front.clone()),
            rear: flatten_list(self.inner.rear.clone()),
        };
        JsValue::from_serde(&state).unwrap()
    }
}

fn flatten_list(
    list: std::rc::Rc<data_structures::linked_list::singly_linked_list::SinglyLinkedList<String>>,
) -> Vec<String> {
    let mut result = Vec::new();
    let mut current = list;

    while let data_structures::linked_list::singly_linked_list::SinglyLinkedList::Cons(
        value,
        next,
    ) = &*current
    {
        result.push(value.clone());
        current = next.clone();
    }

    result
}
