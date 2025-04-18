use std::collections::{BTreeMap, HashMap};
use std::fmt::Debug;
use std::hash::Hash;
use std::sync::RwLock;

use crate::time_indexed::skip_list_with_expiry::traits::{
    ConcurrentIndexBasedExpiry, ConcurrentTimeBasedExpiry,
};

const DEBUGGING_ENABLED: bool = true;
const BUCKET_WIDTH: usize = 100;

#[derive(Debug)]
pub struct BucketBTreeMapExpiry<T>
where
    T: Clone + Ord + Debug + Send + Sync + Hash + 'static,
{
    inner: RwLock<BucketBTreeMapExpiryInner<T>>,
}

#[derive(Debug)]
struct BucketBTreeMapExpiryInner<T>
where
    T: Clone + Ord + Debug + Send + Sync + Hash + 'static,
{
    buckets: BTreeMap<usize, Vec<T>>,
    reverse_index: HashMap<T, usize>,
}

impl<T> BucketBTreeMapExpiry<T>
where
    T: Clone + Ord + Debug + Send + Sync + Hash + 'static,
{
    pub fn new() -> Self {
        Self {
            inner: RwLock::new(BucketBTreeMapExpiryInner {
                buckets: BTreeMap::new(),
                reverse_index: HashMap::new(),
            }),
        }
    }

    /// Disambiguated internal helper
    fn get_all_values(&self) -> Vec<T> {
        let inner = self.inner.read().unwrap();
        inner.buckets.values().flatten().cloned().collect()
    }

    fn get_len(&self) -> usize {
        self.get_all_values().len()
    }

    fn is_empty_internal(&self) -> bool {
        self.get_len() == 0
    }

    fn insert_inner(&self, id: T, level: usize) {
        let mut inner = self.inner.write().unwrap();

        if DEBUGGING_ENABLED {
            eprintln!("Inserting id={:?} into bucket level {}", id, level);
        }

        // Remove previous occurrence if it exists
        if let Some(prev_level) = inner.reverse_index.remove(&id) {
            if DEBUGGING_ENABLED {
                eprintln!(
                    "  -> Removing previous entry for id={:?} from level {}",
                    id, prev_level
                );
            }

            if let Some(bucket) = inner.buckets.get_mut(&prev_level) {
                bucket.retain(|x| x != &id);

                if bucket.is_empty() {
                    if DEBUGGING_ENABLED {
                        eprintln!(
                            "  -> Bucket at level {} is now empty, removing it",
                            prev_level
                        );
                    }
                    inner.buckets.remove(&prev_level);
                }
            }
        }

        // Insert into new bucket and update index
        inner.buckets.entry(level).or_default().push(id.clone());
        inner.reverse_index.insert(id, level);
    }

    fn expire_front_internal(&self) -> Vec<T> {
        let mut inner = self.inner.write().unwrap();

        while let Some((&min_level, items)) = inner.buckets.iter_mut().next() {
            if DEBUGGING_ENABLED {
                eprintln!("Checking bucket level {}", min_level);
            }

            if items.is_empty() {
                if DEBUGGING_ENABLED {
                    eprintln!("  -> Bucket at level {} is empty. Removing it.", min_level);
                }
                inner.buckets.remove(&min_level);
                continue;
            }

            if DEBUGGING_ENABLED {
                eprintln!(
                    "  -> Expiring items from bucket level {}: {:?}",
                    min_level, items
                );
            }
            let expired = std::mem::take(items);

            // Now that the items are moved out, remove the empty bucket
            inner.buckets.remove(&min_level);

            // Remove from reverse index
            for id in &expired {
                if DEBUGGING_ENABLED {
                    eprintln!("  -> Removing id {:?} from reverse index", id);
                }
                inner.reverse_index.remove(id);
            }

            return expired;
        }

        if DEBUGGING_ENABLED {
            eprintln!("No non-empty buckets found to expire.");
        }
        vec![]
    }
}

impl<T> ConcurrentIndexBasedExpiry<T> for BucketBTreeMapExpiry<T>
where
    T: Clone + Ord + Debug + Send + Sync + Hash + 'static,
{
    fn insert(&self, id: T, level: usize) {
        self.insert_inner(id, level);
    }

    fn expire_front(&self) -> Vec<T> {
        self.expire_front_internal()
    }

    fn values(&self) -> Vec<T> {
        self.get_all_values()
    }

    fn len(&self) -> usize {
        self.get_len()
    }

    fn is_empty(&self) -> bool {
        self.is_empty_internal()
    }
}

impl<T> ConcurrentTimeBasedExpiry<T> for BucketBTreeMapExpiry<T>
where
    T: Clone + Ord + Debug + Send + Sync + Hash + 'static,
{
    fn insert(&self, id: T, timestamp: i64) {
        let bucket = (timestamp as usize) / BUCKET_WIDTH;
        eprintln!(
            "insert(timestamp={}, id={:?}) → bucket {}",
            timestamp, id, bucket
        );
        self.insert_inner(id, bucket);
    }

    fn expire_front(&self) -> Vec<T> {
        self.expire_front_internal()
    }

    fn values(&self) -> Vec<T> {
        self.get_all_values()
    }

    fn len(&self) -> usize {
        self.get_len()
    }

    fn is_empty(&self) -> bool {
        self.is_empty_internal()
    }
}
