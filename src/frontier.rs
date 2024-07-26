use std::cmp::Reverse;
use std::collections::{BTreeSet, BinaryHeap};

pub struct MinHeap<T>(BinaryHeap<Reverse<T>>)
where
    T: Ord + Clone;

impl<T> MinHeap<T>
where
    T: Ord + Clone,
{
    pub fn new() -> MinHeap<T> {
        MinHeap(BinaryHeap::new())
    }

    pub fn push(&mut self, t: T) {
        self.0.push(Reverse(t));
    }

    pub fn pop(&mut self) -> Option<T> {
        self.0.pop().map(|t| t.0)
    }

    pub fn size(&self) -> usize {
        self.0.len()
    }

    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    pub fn contains(&self, t: &T) -> bool {
        self.0.iter().any(|r| &r.0 == t)
    }

    pub fn iter(&self) -> impl Iterator<Item = &T> {
        self.0.iter().map(|r| &r.0)
    }
}

pub struct MinSet<T>(BTreeSet<T>)
where
    T: Ord + Clone;

impl<T> MinSet<T>
where
    T: Ord + Clone,
{
    pub fn new() -> MinSet<T> {
        MinSet(BTreeSet::new())
    }

    pub fn push(&mut self, t: T) {
        self.0.insert(t);
    }

    pub fn pop(&mut self) -> Option<T> {
        self.0.pop_first()
    }

    pub fn size(&self) -> usize {
        self.0.len()
    }

    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    pub fn contains(&self, t: &T) -> bool {
        self.0.contains(t)
    }

    pub fn iter(&self) -> impl Iterator<Item = &T> {
        self.0.iter()
    }
}
