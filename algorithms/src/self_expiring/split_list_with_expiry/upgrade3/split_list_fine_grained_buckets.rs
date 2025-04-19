use parking_lot::RwLock;
use std::collections::HashMap;
use std::hash::Hash;
use std::sync::Arc;

use crate::self_expiring::traits::ConcurrentTimeBasedExpiry;

#[derive(Debug)]
pub struct SplitListFineGrainedBuckets<T>
where
    T: Clone + Eq + Hash + std::fmt::Debug + Send + Sync + 'static,
{
    buckets: RwLock<HashMap<i64, Arc<RwLock<Vec<T>>>>>,
}

impl<T> SplitListFineGrainedBuckets<T>
where
    T: Clone + Eq + Hash + std::fmt::Debug + Send + Sync + 'static,
{
    pub fn new() -> Self {
        Self {
            buckets: RwLock::new(HashMap::new()),
        }
    }
}

impl<T> ConcurrentTimeBasedExpiry<T> for SplitListFineGrainedBuckets<T>
where
    T: Clone + Eq + Ord + Hash + std::fmt::Debug + Send + Sync + 'static,
{
    fn insert(&self, id: T, timestamp: i64) {
        let bucket_lock = {
            let buckets_read = self.buckets.read();
            if let Some(existing) = buckets_read.get(&timestamp) {
                return existing.write().push(id);
            }
            drop(buckets_read);

            let mut buckets_write = self.buckets.write();
            buckets_write
                .entry(timestamp)
                .or_insert_with(|| Arc::new(RwLock::new(vec![])))
                .clone()
        };

        bucket_lock.write().push(id);
    }

    fn expire_front(&self) -> Vec<T> {
        let mut buckets_write = self.buckets.write();
        if let Some((min_ts, bucket)) = buckets_write
            .iter()
            .min_by_key(|entry| entry.0)
            .map(|(k, v)| (*k, v.clone()))
        {
            let mut items = bucket.write();
            let expired = std::mem::take(&mut *items);
            buckets_write.remove(&min_ts);
            return expired;
        }
        vec![]
    }

    fn values(&self) -> Vec<T> {
        let buckets_read = self.buckets.read();
        let mut all = vec![];
        for bucket in buckets_read.values() {
            all.extend(bucket.read().iter().cloned());
        }
        all
    }

    fn len(&self) -> usize {
        let buckets_read = self.buckets.read();
        buckets_read.values().map(|b| b.read().len()).sum()
    }

    fn is_empty(&self) -> bool {
        self.len() == 0
    }
}
