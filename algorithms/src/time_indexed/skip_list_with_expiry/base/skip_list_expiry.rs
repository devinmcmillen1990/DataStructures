use chrono::Utc;
use std::collections::{BTreeSet, HashMap, VecDeque};
use std::hash::Hash;
use std::sync::{Arc, Mutex};

use crate::time_indexed::skip_list_with_expiry::traits::TimeBasedExpiry;

/// A concurrent, time‑bucketed expiry structure.
/// Each bucket covers `[start_time + n·resolution , start_time + (n+1)·resolution)`;
/// values falling exactly on a boundary are placed in the *next* bucket.
#[derive(Debug, Clone)]
pub struct SkipListExpiry<T: Ord + Clone + Hash + Eq + Send + Sync + 'static> {
    inner: Arc<Mutex<SkipListExpiryInner<T>>>,
}

#[derive(Debug)]
struct SkipListExpiryInner<T: Ord + Clone + Hash + Eq> {
    buckets: VecDeque<BTreeSet<T>>,
    id_to_bucket: HashMap<T, usize>,
    start_time: i64,
    resolution_secs: i64,
}

/// Inherent methods (including constructor)
impl<T: Ord + Clone + Hash + Eq + Send + Sync + 'static> SkipListExpiry<T> {
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
}

/// Time-based expiry trait implementation
impl<T> TimeBasedExpiry<T> for SkipListExpiry<T>
where
    T: Ord + Clone + Hash + Eq + Send + Sync + 'static + std::fmt::Debug,
{
    fn insert(&mut self, id: T, close_time: i64) {
        let mut inner = self.inner.lock().unwrap();

        // Remove previous schedule (overwrite semantics)
        if let Some(old) = inner.id_to_bucket.remove(&id) {
            if let Some(bucket) = inner.buckets.get_mut(old) {
                bucket.remove(&id);
            }
        }

        let diff = close_time - inner.start_time;
        let offset = (diff + inner.resolution_secs - 1) / inner.resolution_secs;

        if offset < 0 || offset as usize >= inner.buckets.len() {
            eprintln!(
                "[SkipListExpiry] Ignoring insert: out of range. id={:?}, time={}, offset={}",
                id, close_time, offset
            );
            return;
        }

        let idx = offset as usize;
        inner.buckets[idx].insert(id.clone());
        inner.id_to_bucket.insert(id, idx);
    }

    fn expire_front(&mut self) -> Vec<T> {
        let mut inner = self.inner.lock().unwrap();

        let expired = inner.buckets.pop_front().unwrap_or_default();
        inner.buckets.push_back(BTreeSet::new());
        inner.start_time += inner.resolution_secs;

        for item in &expired {
            inner.id_to_bucket.remove(item);
        }

        expired.into_iter().collect()
    }

    fn len(&self) -> usize {
        self.inner.lock().unwrap().id_to_bucket.len()
    }

    fn is_empty(&self) -> bool {
        self.len() == 0
    }

    fn values(&self) -> Vec<T> {
        self.inner
            .lock()
            .unwrap()
            .id_to_bucket
            .keys()
            .cloned()
            .collect()
    }
}
