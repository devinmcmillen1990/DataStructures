#[derive(Clone, Debug)]
pub struct VecFunctionalQueue<T: Clone> {
    pub front:  Vec<T>,
    pub rear:   Vec<T>,
}

impl<T: Clone> VecFunctionalQueue<T> {
    pub fn new() -> Self {
        Self {
            front:  Vec::new(),
            rear:   Vec::new(),
        }
    }

    pub fn is_empty(&self) -> bool {
        self.front.is_empty() && self.rear.is_empty()
    }

    pub fn enqueue(&self, item: T) -> Self {
        let mut new = self.clone();
        new.rear.push(item);
        new.check()
    }    

    pub fn dequeue(&self) -> Option<(T, Self)> {
        let mut new = self.clone();
        if new.front.is_empty() {
            if new.rear.is_empty() {
                return None;
            }
            new.front = new.rear.iter().rev().cloned().collect();
            new.rear.clear();
        }
    
        let val = new.front.pop()?;
        Some((val, new))
    }    

    fn check(mut self) -> Self {
        if self.front.is_empty() {
            self.front = self.rear.iter().rev().cloned().collect();
            self.rear.clear();
        }
        self
    }
}