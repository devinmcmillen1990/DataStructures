use wasm_bindgen::prelude::*;
use data_structures::queue::vec_functional_queue::VecFunctionalQueue;
use serde::{Serialize, Deserialize};

/*
    TODO:
        5. Show Algorithm pseudo-code
        6. Show Original Source material
        7. Setup SEO to make this searchable.
*/

#[derive(Serialize, Deserialize)]
pub struct QueueState {
    front: Vec<String>,
    rear: Vec<String>,
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
            },
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