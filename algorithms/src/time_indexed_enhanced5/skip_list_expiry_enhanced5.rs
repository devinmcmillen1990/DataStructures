use std::collections::{BTreeMap, HashMap, HashSet};
use std::sync::RwLock;

/// A thread-safe skip list expiry structure using BTreeMap and HashSet.
/// This structure maps expiration time to sets of IDs to be expired.
/// It uses a resolution to round timestamps and ensure consistent bucket assignment.
pub struct EnhancedSkipListExpiry5<T>
where
    T: Clone + Eq + std::hash::Hash + Ord,
{
    resolution: usize,
    inner: RwLock<EnhancedSkipListExpiry5Inner<T>>,
}

struct EnhancedSkipListExpiry5Inner<T>
where
    T: Clone + Eq + std::hash::Hash + Ord,
{
    buckets: BTreeMap<usize, HashSet<T>>,
    id_to_expiry: HashMap<T, usize>,
}

impl<T> EnhancedSkipListExpiry5<T>
where
    T: Clone + Eq + std::hash::Hash + Ord,
{
    /// Creates a new instance with a specified resolution.
    pub fn new(resolution: usize) -> Self {
        Self {
            resolution,
            inner: RwLock::new(EnhancedSkipListExpiry5Inner {
                buckets: BTreeMap::new(),
                id_to_expiry: HashMap::new(),
            }),
        }
    }

    /// Inserts an item with an expiration time. If the item already exists, it is replaced.
    pub fn insert(&self, id: T, expire_at: usize) {
        let rounded = (expire_at / self.resolution) * self.resolution;
        let mut inner = self.inner.write().unwrap();

        if let Some(prev_expiry) = inner.id_to_expiry.insert(id.clone(), rounded) {
            if let Some(set) = inner.buckets.get_mut(&prev_expiry) {
                set.remove(&id);
            }
        }

        inner
            .buckets
            .entry(rounded)
            .or_insert_with(HashSet::new)
            .insert(id);
    }

    /// Ticks the structure forward to a given timestamp and returns expired items.
    pub fn tick(&self, now: usize) -> Vec<T> {
        let mut inner = self.inner.write().unwrap();
        let rounded = (now / self.resolution) * self.resolution;
        let mut expired_items = vec![];

        let expired_keys: Vec<usize> = inner
            .buckets
            .range(..=rounded)
            .map(|(&time, _)| time)
            .collect();

        for key in expired_keys {
            if let Some(set) = inner.buckets.remove(&key) {
                for item in set {
                    inner.id_to_expiry.remove(&item);
                    expired_items.push(item);
                }
            }
        }

        expired_items
    }

    /// Returns the current number of items in the expiry structure.
    pub fn len(&self) -> usize {
        let inner = self.inner.read().unwrap();
        inner.id_to_expiry.len()
    }

    /// Returns true if the expiry structure is empty.
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }
}
