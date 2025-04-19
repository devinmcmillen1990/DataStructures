use std::collections::BTreeMap;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::{Arc, RwLock};

use crate::self_expiring::traits::ConcurrentTimeBasedExpiry;

/// A prototype structure designed for real-time expiry using thread-local queues
#[derive(Debug)]
pub struct PrototypeThreadLocalExpiry<T>
where
    T: Clone + Ord + std::fmt::Debug + Send + Sync + 'static,
{
    thread_count: usize,
    selector: AtomicUsize,
    queues: Vec<Arc<LocalQueue<T>>>,
}

#[derive(Debug)]
pub struct LocalQueue<T>
where
    T: Clone + Ord + std::fmt::Debug + Send + Sync + 'static,
{
    buckets: RwLock<BTreeMap<i64, Vec<T>>>,
}

impl<T> PrototypeThreadLocalExpiry<T>
where
    T: Clone + Ord + std::fmt::Debug + Send + Sync + 'static,
{
    pub fn new(num_threads: usize) -> Self {
        let queues = (0..num_threads)
            .map(|_| {
                Arc::new(LocalQueue {
                    buckets: RwLock::new(BTreeMap::new()),
                })
            })
            .collect();

        Self {
            thread_count: num_threads,
            selector: AtomicUsize::new(0),
            queues,
        }
    }

    fn get_queue(&self) -> Arc<LocalQueue<T>> {
        let idx = self.selector.fetch_add(1, Ordering::Relaxed) % self.thread_count;
        Arc::clone(&self.queues[idx])
    }
}

impl<T> ConcurrentTimeBasedExpiry<T> for PrototypeThreadLocalExpiry<T>
where
    T: Clone + Ord + std::fmt::Debug + Send + Sync + 'static,
{
    fn insert(&self, id: T, timestamp: i64) {
        let queue = self.get_queue();
        let mut buckets = queue.buckets.write().unwrap();
        buckets.entry(timestamp).or_default().push(id);
    }

    fn expire_front(&self) -> Vec<T> {
        let mut expired_all = vec![];
        for queue in &self.queues {
            let mut buckets = queue.buckets.write().unwrap();
            if let Some((&ts, items)) = buckets.iter_mut().next() {
                let expired = std::mem::take(items);
                buckets.remove(&ts);
                expired_all.extend(expired);
            }
        }
        expired_all
    }

    fn values(&self) -> Vec<T> {
        self.queues
            .iter()
            .flat_map(|q| {
                let buckets = q.buckets.read().unwrap();
                buckets.values().flatten().cloned().collect::<Vec<_>>()
            })
            .collect()
    }

    fn len(&self) -> usize {
        self.queues
            .iter()
            .map(|q| {
                let buckets = q.buckets.read().unwrap();
                buckets.values().map(Vec::len).sum::<usize>()
            })
            .sum()
    }

    fn is_empty(&self) -> bool {
        self.len() == 0
    }
}
