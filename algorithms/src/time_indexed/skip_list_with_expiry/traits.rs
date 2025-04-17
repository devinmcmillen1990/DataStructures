/// ✅ For structures that expire based on position, index, or tier
pub trait IndexBasedExpiry<T>: Send + Sync + std::fmt::Debug
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

/// ✅ For expiration strategies based on real-time (Unix timestamp, etc.)
pub trait TimeBasedExpiry<T>: Send + Sync + std::fmt::Debug
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
