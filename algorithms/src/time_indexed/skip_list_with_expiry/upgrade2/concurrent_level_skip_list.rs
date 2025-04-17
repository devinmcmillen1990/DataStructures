use crate::time_indexed::skip_list_with_expiry::traits::ConcurrentIndexBasedExpiry;
use std::collections::BTreeSet;
use std::fmt::Debug;
use std::sync::{Arc, RwLock};

const DEBUGGING_ENABLED: bool = true;

#[derive(Debug)]
pub struct ConcurrentLevelSkipList<T>
where
    T: Clone + Ord + Debug + Send + Sync + 'static,
{
    buckets: Vec<Arc<RwLock<BTreeSet<T>>>>,
}

impl<T> ConcurrentLevelSkipList<T>
where
    T: Clone + Ord + Debug + Send + Sync + 'static,
{
    pub fn new(levels: usize) -> Self {
        let mut buckets = Vec::with_capacity(levels);
        for _ in 0..levels {
            buckets.push(Arc::new(RwLock::new(BTreeSet::new())));
        }
        Self { buckets }
    }
}

impl<T> ConcurrentIndexBasedExpiry<T> for ConcurrentLevelSkipList<T>
where
    T: Clone + Ord + Debug + Send + Sync + 'static,
{
    fn insert(&self, id: T, level: usize) {
        if level >= self.buckets.len() && DEBUGGING_ENABLED {
            eprintln!("Insert level {} out of bounds. Using max level {}.", level, self.buckets.len() - 1);
        }

        let insertion_level = if level >= self.buckets.len() {
            self.buckets.len() - 1
        } else {
            level
        };

        if let Some(bucket) = self.buckets.get(insertion_level) {
            let mut guard = bucket.write().unwrap();
            guard.insert(id);
        }
    }

    fn expire_front(&self) -> Vec<T> {
        for bucket in &self.buckets {
            let mut guard = bucket.write().unwrap();
            if !guard.is_empty() {
                let expired: Vec<T> = guard.iter().cloned().collect();
                guard.clear();
                return expired;
            }
        }
        vec![]
    }

    fn values(&self) -> Vec<T> {
        let mut all = vec![];
        for bucket in &self.buckets {
            let guard = bucket.read().unwrap();
            all.extend(guard.iter().cloned());
        }
        all
    }

    fn len(&self) -> usize {
        self.values().len()
    }

    fn is_empty(&self) -> bool {
        self.values().is_empty()
    }
}
