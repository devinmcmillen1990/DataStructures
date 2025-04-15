use data_structures::queue::vec_functional_queue::VecFunctionalQueue;
use serde::{Deserialize, Serialize};
use serde_wasm_bindgen::to_value;
use wasm_bindgen::prelude::*;

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
                to_value(&val).unwrap()
            }
            None => JsValue::NULL,
        }
    }

    pub fn state(&self) -> JsValue {
        let state = QueueState {
            front: self.inner.front.clone(),
            rear: self.inner.rear.clone(),
        };
        to_value(&state).unwrap()
    }
}
