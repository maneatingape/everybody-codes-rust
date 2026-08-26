use std::cmp::Ordering;
use std::collections::BinaryHeap;

struct Wrapper<K: Ord, V> {
    key: K,
    value: V,
}

impl<K: Ord, V> PartialEq for Wrapper<K, V> {
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        self.key == other.key
    }
}

impl<K: Ord, V> Eq for Wrapper<K, V> {}

impl<K: Ord, V> PartialOrd for Wrapper<K, V> {
    #[inline]
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl<K: Ord, V> Ord for Wrapper<K, V> {
    #[inline]
    fn cmp(&self, other: &Self) -> Ordering {
        other.key.cmp(&self.key)
    }
}

#[derive(Default)]
pub struct MinHeap<K: Ord, V> {
    heap: BinaryHeap<Wrapper<K, V>>,
}

impl<K: Ord, V> MinHeap<K, V> {
    #[must_use]
    pub fn new() -> Self {
        Self { heap: BinaryHeap::new() }
    }

    #[must_use]
    pub fn with_capacity(capacity: usize) -> Self {
        Self { heap: BinaryHeap::with_capacity(capacity) }
    }

    #[must_use]
    pub fn from<const N: usize>(arr: [(K, V); N]) -> Self {
        Self { heap: BinaryHeap::from(arr.map(|(key, value)| Wrapper { key, value })) }
    }

    #[inline]
    pub fn push(&mut self, key: K, value: V) {
        self.heap.push(Wrapper { key, value });
    }

    #[inline]
    #[must_use]
    pub fn pop(&mut self) -> Option<(K, V)> {
        self.heap.pop().map(|w| (w.key, w.value))
    }
}
