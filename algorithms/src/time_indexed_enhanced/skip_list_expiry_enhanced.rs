use std::collections::HashMap;
use std::hash::Hash;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::{Arc, RwLock};

#[derive(Debug, Clone)]
pub struct EnhancedSkipListExpiry<T: Clone + Hash + Eq + Send + Sync + 'static> {
    inner: Arc<RwLock<EnhancedSkipListInner<T>>>,
}

#[derive(Debug)]
struct EnhancedSkipListInner<T: Clone + Eq + Hash> {
    buckets: Vec<Vec<T>>,
    id_to_bucket: HashMap<T, usize>,
    bucket_time: AtomicUsize,
    resolution_secs: usize,
    capacity: usize,
}

impl<T: Clone + Hash + Eq + Send + Sync + 'static> EnhancedSkipListExpiry<T> {
    pub fn new(num_buckets: usize, resolution_secs: usize) -> Self {
        Self::with_start_time(num_buckets, resolution_secs, 0)
    }

    pub fn with_start_time(num_buckets: usize, resolution_secs: usize, start_time: usize) -> Self {
        Self {
            inner: Arc::new(RwLock::new(EnhancedSkipListInner {
                buckets: vec![Vec::with_capacity(32); num_buckets],
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

        if let Some(offset) = expire_at.checked_sub(now).map(|diff| diff / inner.resolution_secs) {
            if offset < inner.capacity {
                let bucket = &mut inner.buckets[offset];
                if !bucket.contains(&id) {
                    bucket.push(id.clone());
                    inner.id_to_bucket.insert(id, offset);
                }
            }
        }
    }

    pub fn tick(&self) -> Vec<T> {
        let mut inner = self.inner.write().unwrap();
        let expired = std::mem::take(&mut inner.buckets[0]);
        inner.buckets.rotate_left(1);
        
        let last_index = inner.capacity - 1;
        inner.buckets[last_index] = Vec::with_capacity(32);
    
        inner
            .bucket_time
            .fetch_add(inner.resolution_secs, Ordering::Relaxed);
        for id in &expired {
            inner.id_to_bucket.remove(id);
        }
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
