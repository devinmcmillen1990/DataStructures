use std::collections::BTreeMap;
use std::fmt::Debug;
use std::hash::Hash;
use std::sync::{Arc, RwLock};

use crate::self_expiring::split_list_with_expiry::traits::ConcurrentTimeBasedExpiry;

/// SplitListLinkedBuckets groups elements into buckets by timestamp in sorted order.
/// Internally uses a BTreeMap<i64, Vec<T>> to maintain ordering and bucket grouping.
#[derive(Debug)]
pub struct SplitListLinkedBuckets<T>
where
    T: Clone + Ord + Debug + Send + Sync + Hash + 'static,
{
    inner: Arc<RwLock<BTreeMap<i64, Vec<T>>>>,
}

impl<T> SplitListLinkedBuckets<T>
where
    T: Clone + Ord + Debug + Send + Sync + Hash + 'static,
{
    pub fn new() -> Self {
        Self {
            inner: Arc::new(RwLock::new(BTreeMap::new())),
        }
    }
}

impl<T> ConcurrentTimeBasedExpiry<T> for SplitListLinkedBuckets<T>
where
    T: Clone + Ord + Debug + Send + Sync + Hash + 'static,
{
    fn insert(&self, id: T, timestamp: i64) {
        let mut guard = self.inner.write().unwrap();
        guard.entry(timestamp).or_default().push(id);
    }

    fn expire_front(&self) -> Vec<T> {
        let mut guard = self.inner.write().unwrap();
        while let Some((&ts, items)) = guard.iter_mut().next() {
            if items.is_empty() {
                guard.remove(&ts);
                continue;
            }

            let expired = std::mem::take(items);
            guard.remove(&ts);
            return expired;
        }
        vec![]
    }

    fn values(&self) -> Vec<T> {
        let guard = self.inner.read().unwrap();
        guard.values().flatten().cloned().collect()
    }

    fn len(&self) -> usize {
        let guard = self.inner.read().unwrap();
        guard.values().map(Vec::len).sum()
    }

    fn is_empty(&self) -> bool {
        self.len() == 0
    }
}
