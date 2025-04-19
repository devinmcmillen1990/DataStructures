use crate::self_expiring::traits::ConcurrentTimeBasedExpiry;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::fmt::Debug;
use std::marker::PhantomData;

thread_local! {
    static LOCAL_BUCKETS: RefCell<BTreeMap<i64, Vec<String>>> = RefCell::new(BTreeMap::new());
}

#[derive(Debug)]
pub struct SplitListFanoutThreadLocalExpiry<T>
where
    T: Clone + Ord + Debug + Send + Sync + 'static,
{
    _phantom: PhantomData<T>,
}

impl<T> SplitListFanoutThreadLocalExpiry<T>
where
    T: Clone + Ord + Debug + Send + Sync + 'static,
{
    pub fn new() -> Self {
        Self {
            _phantom: PhantomData,
        }
    }
}

impl ConcurrentTimeBasedExpiry<String> for SplitListFanoutThreadLocalExpiry<String> {
    fn insert(&self, id: String, timestamp: i64) {
        LOCAL_BUCKETS.with(|buckets| {
            let mut map = buckets.borrow_mut();
            map.entry(timestamp).or_default().push(id);
        });
    }

    fn expire_front(&self) -> Vec<String> {
        LOCAL_BUCKETS.with(|buckets| {
            let mut map = buckets.borrow_mut();
            if let Some((&ts, items)) = map.iter_mut().next() {
                let expired = std::mem::take(items);
                map.remove(&ts);
                return expired;
            }
            vec![]
        })
    }

    fn values(&self) -> Vec<String> {
        LOCAL_BUCKETS.with(|buckets| buckets.borrow().values().flatten().cloned().collect())
    }

    fn len(&self) -> usize {
        LOCAL_BUCKETS.with(|buckets| buckets.borrow().values().map(Vec::len).sum())
    }

    fn is_empty(&self) -> bool {
        self.len() == 0
    }
}
