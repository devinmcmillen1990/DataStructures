use chrono::Utc;
use data_structures::time_indexed::skip_list_expiry::SkipListExpiry;
use serde::{Deserialize, Serialize};
use serde_wasm_bindgen::to_value;
use wasm_bindgen::prelude::*;

#[derive(Serialize, Deserialize)]
pub struct SkipListState {
    pub values: Vec<String>,
}

#[wasm_bindgen]
pub struct SkipListExpiryVisual {
    inner: SkipListExpiry<String>,
}

#[wasm_bindgen]
impl SkipListExpiryVisual {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Self {
        // 64 buckets, 1 second resolution
        Self {
            inner: SkipListExpiry::new(64, 1),
        }
    }

    #[wasm_bindgen]
    pub fn insert(&mut self, value: String, ttl_secs: u64) {
        let expire_at = Utc::now().timestamp() + ttl_secs as i64;
        self.inner.insert(value, expire_at);
    }

    #[wasm_bindgen]
    pub fn tick(&mut self) {
        self.inner.expire_front();
    }

    #[wasm_bindgen]
    pub fn state(&self) -> JsValue {
        let values = self.inner.values();
        to_value(&SkipListState { values }).unwrap()
    }
}
