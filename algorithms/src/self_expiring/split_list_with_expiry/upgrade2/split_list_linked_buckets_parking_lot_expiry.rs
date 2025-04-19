use parking_lot::RwLock;
use std::collections::BTreeMap;
use std::fmt::Debug;

use crate::self_expiring::traits::ConcurrentTimeBasedExpiry;

#[derive(Debug)]
pub struct SplitListLinkedBucketsParkingLotExpiry<T>
where
    T: Clone + Ord + Debug + Send + Sync + 'static,
{
    inner: RwLock<BTreeMap<i64, Vec<T>>>,
}

impl<T> SplitListLinkedBucketsParkingLotExpiry<T>
where
    T: Clone + Ord + Debug + Send + Sync + 'static,
{
    pub fn new() -> Self {
        Self {
            inner: RwLock::new(BTreeMap::new()),
        }
    }
}

impl<T> ConcurrentTimeBasedExpiry<T> for SplitListLinkedBucketsParkingLotExpiry<T>
where
    T: Clone + Ord + Debug + Send + Sync + 'static,
{
    fn insert(&self, id: T, timestamp: i64) {
        let mut map = self.inner.write();
        map.entry(timestamp).or_default().push(id);
    }

    fn expire_front(&self) -> Vec<T> {
        let mut map = self.inner.write();
        if let Some((&ts, items)) = map.iter_mut().next() {
            let expired = std::mem::take(items);
            map.remove(&ts);
            return expired;
        }
        vec![]
    }

    fn values(&self) -> Vec<T> {
        let map = self.inner.read();
        map.values().flatten().cloned().collect()
    }

    fn len(&self) -> usize {
        let map = self.inner.read();
        map.values().map(Vec::len).sum()
    }

    fn is_empty(&self) -> bool {
        self.len() == 0
    }
}
