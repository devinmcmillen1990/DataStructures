use chrono::Utc;
use std::collections::{BTreeSet, HashMap, VecDeque};
use std::hash::Hash;

#[derive(Debug, Clone)]
pub struct SegmentedHeap<T> {
    inner: std::sync::Arc<std::sync::Mutex<SegmentedHeapInner<T>>>,
}

#[derive(Debug)]
struct SegmentedHeapInner<T> {
    buckets: VecDeque<BTreeSet<T>>,
    id_to_bucket: HashMap<T, usize>,
    bucket_time: i64,
    resolution_secs: i64,
}

impl<T> SegmentedHeap<T>
where
    T: Ord + Clone + Hash + Eq + Send + 'static,
{
    pub fn new(num_buckets: usize, resolution_secs: i64) -> Self {
        let buckets = VecDeque::from(vec![BTreeSet::new(); num_buckets]);
        Self {
            inner: std::sync::Arc::new(std::sync::Mutex::new(SegmentedHeapInner {
                buckets,
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
        for key in &expired {
            inner.id_to_bucket.remove(key);
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
