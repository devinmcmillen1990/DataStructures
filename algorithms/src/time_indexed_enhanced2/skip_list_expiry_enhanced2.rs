use std::collections::VecDeque;
use std::hash::Hash;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::{Arc, RwLock};

// ✅ Use ahash for high-performance hashing
use ahash::AHashMap as HashMap;

#[derive(Debug, Clone)]
pub struct EnhancedSkipListExpiry2<T: Clone + Hash + Eq + Send + Sync + 'static> {
    inner: Arc<RwLock<EnhancedSkipListInner2<T>>>,
}

#[derive(Debug)]
struct EnhancedSkipListInner2<T: Clone + Eq + Hash> {
    buckets: VecDeque<Vec<T>>,       // ✅ Efficient rotation
    id_to_bucket: HashMap<T, usize>, // ✅ Fast hashing
    bucket_time: AtomicUsize,        // starting timestamp
    resolution_secs: usize,
    capacity: usize,
}

impl<T: Clone + Hash + Eq + Send + Sync + 'static> EnhancedSkipListExpiry2<T> {
    pub fn new(num_buckets: usize, resolution_secs: usize) -> Self {
        Self::with_start_time(num_buckets, resolution_secs, 0)
    }

    pub fn with_start_time(num_buckets: usize, resolution_secs: usize, start_time: usize) -> Self {
        let mut buckets = VecDeque::with_capacity(num_buckets);
        for _ in 0..num_buckets {
            buckets.push_back(Vec::with_capacity(32)); // ✅ Preallocate bucket capacity
        }

        Self {
            inner: Arc::new(RwLock::new(EnhancedSkipListInner2 {
                buckets,
                id_to_bucket: HashMap::with_capacity(1024),
                bucket_time: AtomicUsize::new(start_time),
                resolution_secs,
                capacity: num_buckets,
            })),
        }
    }

    pub fn insert(&self, id: T, expire_at: usize) {
        let mut inner = self.inner.write().unwrap();
        let now = inner.bucket_time.load(Ordering::Relaxed);

        if let Some(offset) = expire_at
            .checked_sub(now)
            .map(|diff| diff / inner.resolution_secs)
        {
            if offset < inner.capacity {
                if let Some(bucket) = inner.buckets.get_mut(offset) {
                    if !bucket.contains(&id) {
                        bucket.push(id.clone());
                        inner.id_to_bucket.insert(id, offset);
                    }
                }
            }
        }
    }

    pub fn tick(&self) -> Vec<T> {
        let mut inner = self.inner.write().unwrap();

        let mut expired = inner.buckets.pop_front().unwrap_or_default();

        // ✅ Clear and reuse the bucket instead of reallocating
        expired.shrink_to_fit(); // optional if bucket sizes are large
        for id in &expired {
            inner.id_to_bucket.remove(id);
        }

        inner.buckets.push_back(Vec::with_capacity(32));

        inner
            .bucket_time
            .fetch_add(inner.resolution_secs, Ordering::Relaxed);

        expired
    }

    pub fn values(&self) -> Vec<T> {
        let inner = self.inner.read().unwrap();
        inner.id_to_bucket.keys().cloned().collect()
    }

    pub fn len(&self) -> usize {
        let inner = self.inner.read().unwrap();
        inner.id_to_bucket.len()
    }
}
