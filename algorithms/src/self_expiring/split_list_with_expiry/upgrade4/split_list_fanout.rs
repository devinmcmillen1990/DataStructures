use crate::self_expiring::traits::ConcurrentTimeBasedExpiry;
use std::collections::BTreeMap;
use std::sync::{
    mpsc::{self, Receiver, Sender},
    Arc, RwLock,
};
use std::thread;

#[derive(Debug)]
pub struct SplitListFanoutExpiry<T>
where
    T: Clone + Ord + std::fmt::Debug + Send + Sync + 'static,
{
    sender: Sender<Task<T>>,
    map: Arc<RwLock<BTreeMap<i64, Vec<T>>>>,
}

#[derive(Debug)]
struct Task<T>
where
    T: Clone + Ord + std::fmt::Debug + Send + Sync + 'static,
{
    id: T,
    timestamp: i64,
}

impl<T> SplitListFanoutExpiry<T>
where
    T: Clone + Ord + std::fmt::Debug + Send + Sync + 'static,
{
    pub fn new(num_workers: usize) -> Self {
        let (sender, receiver): (Sender<Task<T>>, Receiver<Task<T>>) = mpsc::channel();
        let map = Arc::new(RwLock::new(BTreeMap::<i64, Vec<T>>::new()));

        let receiver = Arc::new(std::sync::Mutex::new(receiver));

        for _ in 0..num_workers {
            let rx = Arc::clone(&receiver);
            let map_clone = Arc::clone(&map);

            thread::spawn(move || {
                let mut batch = Vec::with_capacity(256);
                loop {
                    // Block until at least one task is received
                    if let Ok(first) = rx.lock().unwrap().recv() {
                        batch.push(first);

                        // Drain remaining tasks quickly (non-blocking)
                        while let Ok(task) = rx.lock().unwrap().try_recv() {
                            batch.push(task);
                            if batch.len() >= 256 {
                                break;
                            }
                        }

                        // Process batch
                        let mut guard = map_clone.write().unwrap();
                        for task in batch.drain(..) {
                            guard.entry(task.timestamp).or_default().push(task.id);
                        }
                    }
                }
            });
        }

        Self { sender, map }
    }
}

impl<T> ConcurrentTimeBasedExpiry<T> for SplitListFanoutExpiry<T>
where
    T: Clone + Ord + std::fmt::Debug + Send + Sync + 'static,
{
    fn insert(&self, id: T, timestamp: i64) {
        let task = Task { id, timestamp };
        let _ = self.sender.send(task);
    }

    fn expire_front(&self) -> Vec<T> {
        let mut guard = self.map.write().unwrap();
        if let Some((&ts, items)) = guard.iter_mut().next() {
            let expired = std::mem::take(items);
            guard.remove(&ts);
            return expired;
        }
        vec![]
    }

    fn values(&self) -> Vec<T> {
        let guard = self.map.read().unwrap();
        guard.values().flatten().cloned().collect()
    }

    fn len(&self) -> usize {
        let guard = self.map.read().unwrap();
        guard.values().map(Vec::len).sum()
    }

    fn is_empty(&self) -> bool {
        self.len() == 0
    }
}
