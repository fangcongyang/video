
use std::collections::VecDeque;

pub struct CustomQueue<T> {
    vec: VecDeque<T>,
}

impl<T: Clone> CustomQueue<T> {
    pub fn new() -> Self {
        CustomQueue {
            vec: VecDeque::new(),
        }
    }

    pub fn size(&self) -> usize {
        self.vec.len()
    }

    pub fn enqueue(&mut self, e: T) {
        self.vec.push_back(e)
    }

    pub fn dequeue(&mut self) -> Option<T> {
        self.vec.pop_front()
    }
}