use std::collections::{BTreeSet, HashMap, VecDeque};
use std::hash::Hash;
use std::sync::{Arc, Mutex};

// TODO: Re-evaluate these at a later point because there were a ton of changes.

pub struct TimingWheel<T> {
    inner: Arc<Mutex<TimingWheelInner<T>>>,
}

struct TimingWheelInner<T> {
    slots: VecDeque<BTreeSet<T>>,
    item_to_slot: HashMap<T, usize>,
    current_slot: usize,
    resolution_secs: i64,
    start_time: i64,
}

impl<T: Clone + Eq + Hash + Ord> TimingWheel<T> {
    pub fn new(num_slots: usize, resolution_secs: i64) -> Self {
        let slots = VecDeque::from(vec![BTreeSet::new(); num_slots]);
        let item_to_slot = HashMap::new();
        let start_time = chrono::Utc::now().timestamp();

        TimingWheel {
            inner: Arc::new(Mutex::new(TimingWheelInner {
                slots,
                item_to_slot,
                current_slot: 0,
                resolution_secs,
                start_time,
            })),
        }
    }

    pub fn insert(&self, item: T, timestamp: i64) {
        let mut inner = self.inner.lock().unwrap();
        let elapsed = timestamp - inner.start_time;
        if elapsed < 0 {
            return;
        }

        let offset = (elapsed / inner.resolution_secs) as usize;
        let index = (inner.current_slot + offset) % inner.slots.len();

        inner.slots[index].insert(item.clone());
        inner.item_to_slot.insert(item, index);
    }

    pub fn tick(&self) -> Vec<T> {
        let mut inner = self.inner.lock().unwrap();
        let current_slot = inner.current_slot; // <- fixes borrow checker issue
        let expired = std::mem::take(&mut inner.slots[current_slot]);
        inner.current_slot = (inner.current_slot + 1) % inner.slots.len();
        for val in &expired {
            inner.item_to_slot.remove(val);
        }
        expired.into_iter().collect()
    }

    pub fn len(&self) -> usize {
        let inner = self.inner.lock().unwrap();
        inner.item_to_slot.len()
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    pub fn current_time(&self) -> i64 {
        let inner = self.inner.lock().unwrap();
        inner.start_time
    }

    pub fn start_time(&self) -> i64 {
        let inner = self.inner.lock().unwrap();
        inner.start_time
    }

    pub fn resolution_secs(&self) -> i64 {
        let inner = self.inner.lock().unwrap();
        inner.resolution_secs
    }
}

impl<T: Clone + Eq + Hash + Ord> Clone for TimingWheel<T> {
    fn clone(&self) -> Self {
        TimingWheel {
            inner: Arc::clone(&self.inner),
        }
    }
}
