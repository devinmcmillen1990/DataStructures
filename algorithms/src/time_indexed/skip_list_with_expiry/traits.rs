//! Expiry Traits for Index-Based and Time-Based Skip Lists and Variants

/// Single-threaded index-based expiry interface
pub trait IndexBasedExpiry<T>: std::fmt::Debug
where
    T: Clone + Eq + Ord + std::fmt::Debug + Send + Sync + 'static,
{
    fn insert(&mut self, id: T, level: usize);
    fn expire_front(&mut self) -> Vec<T>;
    fn values(&self) -> Vec<T>;

    fn len(&self) -> usize {
        self.values().len()
    }

    fn is_empty(&self) -> bool {
        self.values().is_empty()
    }
}

/// Thread-safe index-based expiry interface
pub trait ConcurrentIndexBasedExpiry<T>: Send + Sync + std::fmt::Debug
where
    T: Clone + Eq + Ord + std::fmt::Debug + Send + Sync + 'static,
{
    fn insert(&self, id: T, level: usize);
    fn expire_front(&self) -> Vec<T>;
    fn values(&self) -> Vec<T>;

    fn len(&self) -> usize {
        self.values().len()
    }

    fn is_empty(&self) -> bool {
        self.values().is_empty()
    }
}

/// Single-threaded time-based expiry interface
pub trait TimeBasedExpiry<T>: std::fmt::Debug
where
    T: Clone + Eq + Ord + std::fmt::Debug + Send + Sync + 'static,
{
    fn insert(&mut self, id: T, expiry_time: i64);
    fn expire_front(&mut self) -> Vec<T>;
    fn values(&self) -> Vec<T>;

    fn len(&self) -> usize {
        self.values().len()
    }

    fn is_empty(&self) -> bool {
        self.values().is_empty()
    }
}

/// Thread-safe time-based expiry interface
pub trait ConcurrentTimeBasedExpiry<T>: Send + Sync + std::fmt::Debug
where
    T: Clone + Eq + Ord + std::fmt::Debug + Send + Sync + 'static,
{
    fn insert(&self, id: T, expiry_time: i64);
    fn expire_front(&self) -> Vec<T>;
    fn values(&self) -> Vec<T>;

    fn len(&self) -> usize {
        self.values().len()
    }

    fn is_empty(&self) -> bool {
        self.values().is_empty()
    }
}
