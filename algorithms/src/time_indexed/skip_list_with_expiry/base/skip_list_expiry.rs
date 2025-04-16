use chrono::Utc;
use std::collections::{BTreeSet, HashMap, VecDeque};
use std::hash::Hash;
use std::sync::{Arc, Mutex};

/// A concurrent, time‑bucketed wheel.
/// Each bucket covers `[start_time + n·resolution , start_time + (n+1)·resolution)`;
/// values falling exactly on a boundary are placed in the *next* bucket.
#[derive(Debug, Clone)]
pub struct SkipListExpiry<T: Ord + Clone + Hash + Eq + Send + 'static> {
    inner: Arc<Mutex<SkipListExpiryInner<T>>>,
}

#[derive(Debug)]
struct SkipListExpiryInner<T: Ord + Clone + Hash + Eq> {
    buckets: VecDeque<BTreeSet<T>>,
    id_to_bucket: HashMap<T, usize>,
    start_time: i64, // Renamed from bucket_time for clarity
    resolution_secs: i64,
}

impl<T: Ord + Clone + Hash + Eq + Send + 'static + std::fmt::Debug> SkipListExpiry<T> {
    /// Create a new wheel with `num_buckets` slots of length `resolution_secs`.
    pub fn new(num_buckets: usize, resolution_secs: i64) -> Self {
        Self {
            inner: Arc::new(Mutex::new(SkipListExpiryInner {
                buckets: VecDeque::from(vec![BTreeSet::new(); num_buckets]),
                id_to_bucket: HashMap::new(),
                start_time: Utc::now().timestamp(),
                resolution_secs,
            })),
        }
    }

    /// Insert `id` that should expire at absolute unix‐time `close_time`.
    pub fn insert(&self, id: T, close_time: i64) {
        let mut inner = self.inner.lock().unwrap();

        // Remove previous schedule (overwrite‑semantics)
        if let Some(old) = inner.id_to_bucket.remove(&id) {
            if let Some(bucket) = inner.buckets.get_mut(old) {
                bucket.remove(&id);
            }
        }

        // Calculate offset using ceil-division
        let diff = close_time - inner.start_time;
        let offset = (diff + inner.resolution_secs - 1) / inner.resolution_secs;

        if offset < 0 || offset as usize >= inner.buckets.len() {
            // 👇 Helpful for debugging bad inserts
            eprintln!(
                "[SkipListExpiry] Ignoring insert: item out of range. id = {:?}, close_time = {}, start_time = {}, resolution = {}, offset = {}",
                id, close_time, inner.start_time, inner.resolution_secs, offset
            );
            // panic!("Item out of range: {:?}", id); // ← Uncomment for hard-fail in dev
            return;
        }

        let idx = offset as usize;
        inner.buckets[idx].insert(id.clone());
        inner.id_to_bucket.insert(id, idx);
    }

    /// Advance the wheel one slot and return expired items.
    pub fn expire_front(&self) -> Vec<T> {
        let mut inner = self.inner.lock().unwrap();

        let expired = inner.buckets.pop_front().unwrap_or_default();
        inner.buckets.push_back(BTreeSet::new());
        inner.start_time += inner.resolution_secs; // logical clock tick

        for item in &expired {
            inner.id_to_bucket.remove(item);
        }
        expired.into_iter().collect()
    }

    pub fn len(&self) -> usize {
        self.inner.lock().unwrap().id_to_bucket.len()
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    /// Snapshot of currently scheduled ids (unordered).
    pub fn values(&self) -> Vec<T> {
        self.inner
            .lock()
            .unwrap()
            .id_to_bucket
            .keys()
            .cloned()
            .collect()
    }
}
