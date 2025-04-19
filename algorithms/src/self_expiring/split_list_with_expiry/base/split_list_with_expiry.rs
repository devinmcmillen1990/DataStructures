use std::collections::BTreeMap;
use std::fmt::Debug;
use std::sync::RwLock;

use crate::self_expiring::split_list_with_expiry::traits::ConcurrentTimeBasedExpiry;

/// Thread-safe split list structure using BTreeMap for time-based expiry
#[derive(Debug)]
pub struct SplitListExpiry<T>
where
    T: Clone + Ord + Debug + Send + Sync + 'static,
{
    inner: RwLock<BTreeMap<i64, Vec<T>>>,
}

impl<T> SplitListExpiry<T>
where
    T: Clone + Ord + Debug + Send + Sync + 'static,
{
    pub fn new() -> Self {
        Self {
            inner: RwLock::new(BTreeMap::new()),
        }
    }
}

impl<T> ConcurrentTimeBasedExpiry<T> for SplitListExpiry<T>
where
    T: Clone + Ord + Debug + Send + Sync + 'static,
{
    fn insert(&self, id: T, timestamp: i64) {
        let mut map = self.inner.write().unwrap();
        map.entry(timestamp).or_default().push(id);
    }

    fn expire_front(&self) -> Vec<T> {
        let mut map = self.inner.write().unwrap();
        while let Some((&ts, items)) = map.iter_mut().next() {
            if items.is_empty() {
                map.remove(&ts);
                continue;
            }
            let expired = std::mem::take(items);
            map.remove(&ts);
            return expired;
        }
        vec![]
    }

    fn values(&self) -> Vec<T> {
        let map = self.inner.read().unwrap();
        map.values().flatten().cloned().collect()
    }

    fn len(&self) -> usize {
        let map = self.inner.read().unwrap();
        map.values().map(Vec::len).sum()
    }

    fn is_empty(&self) -> bool {
        self.len() == 0
    }
}
