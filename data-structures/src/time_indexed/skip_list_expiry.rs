use chrono::Utc;
use std::collections::{BTreeSet, HashMap, VecDeque};
use std::hash::Hash;
use std::sync::{Arc, Mutex};

/// A concurrent, time-bucketed structure using skip-list-like properties.
/// Each bucket stores a sorted set of items expiring in a time window.
#[derive(Debug, Clone)]
pub struct SkipListExpiry<T: Ord + Clone + Hash + Eq + Send + 'static> {
    inner: Arc<Mutex<SkipListExpiryInner<T>>>,
}

#[derive(Debug)]
struct SkipListExpiryInner<T: Ord + Clone + Hash + Eq> {
    buckets: VecDeque<BTreeSet<T>>,
    id_to_bucket: HashMap<T, usize>,
    bucket_time: i64,
    resolution_secs: i64,
}

impl<T: Ord + Clone + Hash + Eq + Send + 'static> SkipListExpiry<T> {
    pub fn new(num_buckets: usize, resolution_secs: i64) -> Self {
        Self {
            inner: Arc::new(Mutex::new(SkipListExpiryInner {
                buckets: VecDeque::from(vec![BTreeSet::new(); num_buckets]),
                id_to_bucket: HashMap::new(),
                bucket_time: Utc::now().timestamp(),
                resolution_secs,
            })),
        }
    }

    pub fn insert(&self, id: T, close_time: i64) {
        let mut inner = self.inner.lock().unwrap();
        let offset = (close_time - inner.bucket_time) / inner.resolution_secs;
        if offset < 0 || offset as usize >= inner.buckets.len() {
            return;
        }
        let index = offset as usize;
        inner.buckets[index].insert(id.clone());
        inner.id_to_bucket.insert(id, index);
    }

    pub fn expire_front(&self) -> Vec<T> {
        let mut inner = self.inner.lock().unwrap();
        let expired = inner.buckets.pop_front().unwrap_or_default();
        inner.buckets.push_back(BTreeSet::new());
        for item in &expired {
            inner.id_to_bucket.remove(item);
        }
        expired.into_iter().collect()
    }

    pub fn len(&self) -> usize {
        let inner = self.inner.lock().unwrap();
        inner.id_to_bucket.len()
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }
}
