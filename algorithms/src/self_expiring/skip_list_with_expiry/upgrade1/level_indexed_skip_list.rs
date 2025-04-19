use crate::self_expiring::traits::IndexBasedExpiry;
use std::collections::{BTreeSet, HashMap};
use std::hash::Hash;

// TODO: Get better Debugging statements to fully describe the algorithm

const DEBUGGING_ENABLED: bool = false;

/// A non-thread-safe skip list based on levels rather than time.
#[derive(Debug)]
pub struct LevelIndexedSkipList<T: Ord + Clone + Eq + Hash> {
    buckets: Vec<BTreeSet<T>>,
    id_to_level: HashMap<T, usize>,
}

impl<T> LevelIndexedSkipList<T>
where
    T: Ord + Clone + Eq + Hash,
{
    /// Creates a new skip list with the specified number of levels.
    pub fn new(num_levels: usize) -> Self {
        Self {
            buckets: vec![BTreeSet::new(); num_levels],
            id_to_level: HashMap::new(),
        }
    }
}

impl<T> IndexBasedExpiry<T> for LevelIndexedSkipList<T>
where
    T: Clone + Eq + Ord + Hash + std::fmt::Debug + Send + Sync + 'static,
{
    /// Inserts an ID into a specific level. If it already exists, it is overwritten.
    fn insert(&mut self, id: T, level: usize) {
        if level >= self.buckets.len() && DEBUGGING_ENABLED {
            eprintln!(
                "Insert level {} out of bounds. Using max level {}.",
                level,
                self.buckets.len() - 1
            );
        }

        let insertion_level = if level >= self.buckets.len() {
            self.buckets.len() - 1
        } else {
            level
        };

        if let Some(prev_level) = self.id_to_level.get(&id) {
            self.buckets[*prev_level].remove(&id);
        }

        self.buckets[insertion_level].insert(id.clone());
        self.id_to_level.insert(id.clone(), insertion_level);

        if DEBUGGING_ENABLED {
            eprintln!(
                "[LevelIndexedSkipList] Inserted {:?} at level {}",
                id, insertion_level
            );
            eprintln!("[LevelIndexedSkipList] Current buckets: {:?}", self.buckets);
        }
    }

    /// Returns items from the first non-empty bucket and clears it.
    fn expire_front(&mut self) -> Vec<T> {
        for (level_index, bucket) in self.buckets.iter_mut().enumerate() {
            if DEBUGGING_ENABLED {
                println!("Expiring from level {}", level_index);
            }

            if !bucket.is_empty() {
                let expired: Vec<T> = bucket.iter().cloned().collect();
                for id in &expired {
                    self.id_to_level.remove(id);
                }

                if DEBUGGING_ENABLED {
                    println!("Expiring {:?} from level {}", expired, level_index);
                }

                bucket.clear();
                return expired;
            }
        }
        vec![]
    }

    /// Number of items in the structure.
    fn len(&self) -> usize {
        self.id_to_level.len()
    }

    /// Whether any items are being tracked.
    fn is_empty(&self) -> bool {
        self.id_to_level.is_empty()
    }

    /// Snapshot of all currently tracked items.
    fn values(&self) -> Vec<T> {
        self.id_to_level.keys().cloned().collect()
    }
}
