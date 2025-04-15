use chrono::Utc;
use std::collections::{BTreeSet, HashMap, VecDeque};
use std::hash::Hash;
use std::sync::{Arc, Mutex};

/// A time-partitioned circular buffer (timing wheel) for scheduled expiry.
#[derive(Debug, Clone)]
pub struct TimingWheel<T: Ord + Clone + Hash + Eq + Send + 'static> {
    inner: Arc<Mutex<TimingWheelInner<T>>>,
}

#[derive(Debug)]
struct TimingWheelInner<T: Ord + Clone + Hash + Eq> {
    buckets: VecDeque<BTreeSet<T>>,
    id_to_slot: HashMap<T, usize>,
    resolution_secs: i64,
    current_slot: usize,
    base_time: i64,
}

impl<T: Ord + Clone + Hash + Eq + Send + 'static> TimingWheel<T> {
    pub fn new(num_slots: usize, resolution_secs: i64) -> Self {
        Self {
            inner: Arc::new(Mutex::new(TimingWheelInner {
                buckets: VecDeque::from(vec![BTreeSet::new(); num_slots]),
                id_to_slot: HashMap::new(),
                resolution_secs,
                current_slot: 0,
                base_time: Utc::now().timestamp(),
            })),
        }
    }

    /// Insert an item to be expired at `expire_time`
    pub fn insert(&self, id: T, expire_time: i64) {
        let mut inner = self.inner.lock().unwrap();
        let delta_secs = expire_time - inner.base_time;
        if delta_secs < 0 {
            return; // already expired
        }

        let ticks = delta_secs / inner.resolution_secs;
        let slot = (inner.current_slot + (ticks as usize)) % inner.buckets.len();

        inner.buckets[slot].insert(id.clone());
        inner.id_to_slot.insert(id, slot);
    }

    /// Advance the timing wheel by 1 slot and expire items in the new current slot
    pub fn tick(&self) -> Vec<T> {
        let mut inner = self.inner.lock().unwrap();

        // Advance the slot
        inner.current_slot = (inner.current_slot + 1) % inner.buckets.len();
        inner.base_time += inner.resolution_secs;

        // Capture the index before use
        let slot = inner.current_slot;

        // Take and replace the bucket
        let expired = std::mem::take(&mut inner.buckets[slot]);
        for id in &expired {
            inner.id_to_slot.remove(id);
        }

        expired.into_iter().collect()
    }

    pub fn len(&self) -> usize {
        let inner = self.inner.lock().unwrap();
        inner.id_to_slot.len()
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }
}
