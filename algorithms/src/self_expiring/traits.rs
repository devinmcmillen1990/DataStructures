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
