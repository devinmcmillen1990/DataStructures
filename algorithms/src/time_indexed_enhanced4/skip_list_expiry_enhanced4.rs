use indexmap::IndexSet;
use std::collections::BTreeMap;
use std::hash::Hash;
use std::sync::{Arc, RwLock};

/// EnhancedSkipListExpiry4: Tiered SkipList using BTreeMap for fast expiry checks.
/// Expiry times are sorted in ascending order allowing fast range queries
/// and maintaining precision without fixed-size buckets.
#[derive(Clone)]
pub struct EnhancedSkipListExpiry4<T: Eq + Hash + Clone + Ord> {
    inner: Arc<RwLock<Inner<T>>>,
}

struct Inner<T: Eq + Hash + Clone + Ord> {
    expiry_map: BTreeMap<usize, IndexSet<T>>,
    id_to_expiry: IndexSet<(T, usize)>,
    len: usize,
}

impl<T: Eq + Hash + Clone + Ord> EnhancedSkipListExpiry4<T> {
    pub fn new() -> Self {
        Self {
            inner: Arc::new(RwLock::new(Inner {
                expiry_map: BTreeMap::new(),
                id_to_expiry: IndexSet::new(),
                len: 0,
            })),
        }
    }

    pub fn insert(&self, id: T, expiry: usize) {
        let mut inner = self.inner.write().unwrap();

        if inner.id_to_expiry.insert((id.clone(), expiry)) {
            inner.expiry_map.entry(expiry).or_default().insert(id);
            inner.len += 1;
        }
    }

    pub fn tick(&self, now: usize) -> Vec<T> {
        let mut expired_items = Vec::new();
        let mut inner = self.inner.write().unwrap();

        let expired_keys: Vec<usize> = inner.expiry_map.range(..=now).map(|(k, _)| *k).collect();

        for key in expired_keys {
            if let Some(items) = inner.expiry_map.remove(&key) {
                for item in items {
                    inner.id_to_expiry.remove(&(item.clone(), key));
                    expired_items.push(item);
                    inner.len -= 1;
                }
            }
        }

        expired_items
    }

    pub fn len(&self) -> usize {
        self.inner.read().unwrap().len
    }
}
