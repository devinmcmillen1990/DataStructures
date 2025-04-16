use indexmap::IndexSet;
use std::collections::VecDeque;
use std::hash::Hash;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::{Arc, RwLock};

#[derive(Debug, Clone)]
pub struct EnhancedSkipListExpiry3<T: Clone + Hash + Eq + Send + Sync + 'static> {
    inner: Arc<RwLock<EnhancedSkipListInner3<T>>>,
}

#[derive(Debug)]
struct EnhancedSkipListInner3<T: Clone + Eq + Hash> {
    buckets: VecDeque<IndexSet<T>>,
    id_to_bucket: IndexSet<T>,
    bucket_time: AtomicUsize,
    resolution_secs: usize,
    capacity: usize,
}

impl<T: Clone + Hash + Eq + Send + Sync + 'static> EnhancedSkipListExpiry3<T> {
    pub fn new(num_buckets: usize, resolution_secs: usize) -> Self {
        Self::with_start_time(num_buckets, resolution_secs, 0)
    }

    pub fn with_start_time(num_buckets: usize, resolution_secs: usize, start_time: usize) -> Self {
        let mut buckets = VecDeque::with_capacity(num_buckets);
        for _ in 0..num_buckets {
            buckets.push_back(IndexSet::with_capacity(32));
        }

        Self {
            inner: Arc::new(RwLock::new(EnhancedSkipListInner3 {
                buckets,
                id_to_bucket: IndexSet::with_capacity(1024),
                bucket_time: AtomicUsize::new(start_time),
                resolution_secs,
                capacity: num_buckets,
            })),
        }
    }

    pub fn insert(&self, id: T, expire_at: usize) {
        let mut inner = self.inner.write().unwrap();
        let now = inner.bucket_time.load(Ordering::Relaxed);

        if let Some(diff) = expire_at.checked_sub(now) {
            let offset = diff / inner.resolution_secs;
            if offset < inner.capacity {
                if inner.id_to_bucket.insert(id.clone()) {
                    if let Some(bucket) = inner.buckets.get_mut(offset) {
                        bucket.insert(id);
                    }
                }
            }
        }
    }

    pub fn tick(&self) -> Vec<T> {
        let mut inner = self.inner.write().unwrap();
        let expired = inner.buckets.pop_front().unwrap_or_default();

        for id in &expired {
            inner.id_to_bucket.shift_remove(id);
        }

        inner
            .bucket_time
            .fetch_add(inner.resolution_secs, Ordering::Relaxed);

        inner.buckets.push_back(IndexSet::with_capacity(32));
        expired.into_iter().collect()
    }

    pub fn values(&self) -> Vec<T> {
        let inner = self.inner.read().unwrap();
        inner.id_to_bucket.iter().cloned().collect()
    }

    pub fn len(&self) -> usize {
        let inner = self.inner.read().unwrap();
        inner.id_to_bucket.len()
    }
}
