/*
    TODO:
        1. Setup SEO to make this searchable (Maybe JS)
        2. Replace custom CSS with TailWindCSS (Maybe JS)
        3. Ensure is mobile friendly (Maybe JS)
        4. Add "Buy me a Coffee" (Maybe JS)
        5. Add Advertisements (Maybe JS)
*/
use data_structures::time_indexed::timing_wheel::TimingWheel;
use wasm_bindgen::prelude::*;
use serde::{Serialize, Deserialize};
use std::time::{Duration, SystemTime};

#[derive(Serialize, Deserialize)]
pub struct TimingWheelState {
    pub values: Vec<String>,
}

#[wasm_bindgen]
pub struct TimingWheelVisual {
    inner: TimingWheel<String>,
}

#[wasm_bindgen]
impl TimingWheelVisual {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Self {
        Self {
            inner: TimingWheel::new(Duration::from_secs(1), 60),
        }
    }

    pub fn insert(&mut self, value: String, ttl_secs: u64) {
        let expire_at = SystemTime::now()
            .checked_add(Duration::from_secs(ttl_secs))
            .unwrap();
        self.inner.insert(value, expire_at);
    }

    pub fn tick(&mut self) {
        self.inner.tick();
    }

    pub fn state(&self) -> JsValue {
        let values = self.inner.values().cloned().collect::<Vec<_>>();
        JsValue::from_serde(&TimingWheelState { values }).unwrap()
    }
}
