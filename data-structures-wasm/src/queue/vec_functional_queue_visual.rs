use data_structures::queue::vec_functional_queue::VecFunctionalQueue;
use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;

/*
    TODO:
        1. Setup SEO to make this searchable (Maybe JS)
        2. Replace custom CSS with TailWindCSS (Maybe JS)
        3. Ensure is mobile friendly (Maybe JS)
        4. Add "Buy me a Coffee" (Maybe JS)
        5. Add Advertisements (Maybe JS)
*/

#[derive(Serialize, Deserialize)]
pub struct QueueState {
    pub front: Vec<String>,
    pub rear: Vec<String>,
}

#[wasm_bindgen]
pub struct VecFunctionalQueueVisual {
    inner: VecFunctionalQueue<String>,
}

#[wasm_bindgen]
impl VecFunctionalQueueVisual {
    #[wasm_bindgen(constructor)]
    pub fn new() -> VecFunctionalQueueVisual {
        VecFunctionalQueueVisual {
            inner: VecFunctionalQueue::new(),
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
            front: self.inner.front.clone(),
            rear: self.inner.rear.clone(),
        };
        JsValue::from_serde(&state).unwrap()
    }
}
