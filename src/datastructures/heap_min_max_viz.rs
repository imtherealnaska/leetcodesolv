use std::{cmp::Ordering, fmt::Display, marker::PhantomData};

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum HeapType {
    Min,
    Max,
}

pub trait HeapComparator<T> {
    fn compare(&self, a: &T, b: &T) -> Ordering;
}

#[derive(Debug)]
pub(crate) struct DefaultComparator<T>(PhantomData<T>);

impl<T: Ord> HeapComparator<T> for DefaultComparator<T> {
    fn compare(&self, a: &T, b: &T) -> Ordering {
        a.cmp(b)
    }
}

impl<T> Default for DefaultComparator<T> {
    fn default() -> Self {
        DefaultComparator(PhantomData)
    }
}

#[derive(Debug, Clone)]
pub struct Entry<K, V> {
    pub key: K,
    pub value: V,
}

impl<K: Ord, V> PartialEq for Entry<K, V> {
    fn eq(&self, other: &Self) -> bool {
        self.key == other.key
    }
}
impl<K: Ord, V> Eq for Entry<K, V> {}

impl<K: Ord, V> PartialOrd for Entry<K, V> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl<K: Ord, V> Ord for Entry<K, V> {
    fn cmp(&self, other: &Self) -> Ordering {
        self.key.cmp(&other.key)
    }
}

#[derive(Debug)]
pub struct Heap<T, C: HeapComparator<T> = DefaultComparator<T>> {
    items: Vec<T>,
    heap_type: HeapType,
    comparator: C,
}

impl<T: Ord> Default for Heap<T, DefaultComparator<T>> {
    fn default() -> Self {
        Self::new_min()
    }
}

impl<T: Ord> Heap<T, DefaultComparator<T>> {
    pub fn new_min() -> Self {
        Self {
            items: Vec::new(),
            heap_type: HeapType::Min,
            comparator: DefaultComparator::default(),
        }
    }

    pub fn new_max() -> Self {
        Self {
            items: Vec::new(),
            heap_type: HeapType::Max,
            comparator: DefaultComparator::default(),
        }
    }
}

impl<T, C: HeapComparator<T>> Heap<T, C> {
    pub fn with_comparator(comparator: C, heap_type: HeapType) -> Self {
        Self {
            items: Vec::new(),
            heap_type,
            comparator,
        }
    }

    pub fn with_capacity(capacity: usize, comparator: C, heap_type: HeapType) -> Self {
        Self {
            items: Vec::with_capacity(capacity),
            heap_type,
            comparator,
        }
    }

    pub fn len(&self) -> usize {
        self.items.len()
    }

    pub fn is_empty(&self) -> bool {
        self.items.is_empty()
    }

    pub fn clear(&mut self) {
        self.items.clear();
    }

    pub fn peek(&self) -> Option<&T> {
        self.items.first()
    }

    pub fn push(&mut self, item: T) {
        self.items.push(item);
        self.sift_up(self.len() - 1);
    }

    pub fn pop(&mut self) -> Option<T> {
        if self.is_empty() {
            return None;
        }

        let item = self.items.swap_remove(0);
        if !self.is_empty() {
            self.sift_down(0);
        }
        Some(item)
    }

    pub fn compare_items(&self, a_idx: usize, b_idx: usize) -> Ordering {
        let ordering = self
            .comparator
            .compare(&self.items[a_idx], &self.items[b_idx]);
        if self.heap_type == HeapType::Max {
            ordering.reverse()
        } else {
            ordering
        }
    }

    fn parent(index: usize) -> usize {
        (index - 1) / 2
    }

    pub fn update_key(&mut self, index: usize, item: T) -> Result<T, &'static str> {
        if index >= self.len() {
            return Err("Index out of bounds");
        }

        let old_index = std::mem::replace(&mut self.items[index], item);
        let ordering = self.compare_items(index, Self::parent(index));

        match ordering {
            Ordering::Less => self.sift_up(index),
            Ordering::Greater => self.sift_down(index),
            Ordering::Equal => {}
        }

        Ok(old_index)
    }

    pub fn delete(&mut self, index: usize) -> Result<T, &'static str> {
        if index >= self.len() {
            return Err("Index out of bounds");
        }

        let item = if index == self.len() - 1 {
            self.items.pop().unwrap()
        } else {
            let item = self.items.swap_remove(index);
            self.sift_down(index);
            self.sift_up(index);
            item
        };
        Ok(item)
    }

    fn left_child(index: usize) -> usize {
        2 * index + 1
    }

    fn right_child(index: usize) -> usize {
        2 * index + 2
    }

    fn sift_up(&mut self, mut index: usize) {
        while index > 0 {
            let parent = Self::parent(index);
            if self.compare_items(index, parent) == Ordering::Less {
                self.items.swap(index, parent);
                index = parent;
            } else {
                break;
            }
        }
    }

    fn sift_down(&mut self, mut index: usize) {
        loop {
            let left = Self::left_child(index);
            let right = Self::right_child(index);
            let mut target = index;

            if left < self.len() && self.compare_items(left, target) == Ordering::Less {
                target = left;
            }

            if right < self.len() && self.compare_items(right, target) == Ordering::Less {
                target = right;
            }

            if target == index {
                break;
            }

            self.items.swap(index, target);
            index = target;
        }
    }

    fn merge(&mut self, other: Self) {
        if self.heap_type != other.heap_type {
            panic!("Cannot merge heaps of different type");
        }

        for item in other.items {
            self.push(item);
        }
    }

    pub fn visualize(&self) -> String
    where
        T: Display,
    {
        if self.is_empty() {
            return String::from("Empty heap");
        }

        let mut result = String::new();
        let mut level = 0;
        let mut level_count = 0;
        let mut max_level_size = 1;

        for (i, item) in self.items.iter().enumerate() {
            if level_count == 0 {
                result.push('\n');
                result.push_str(&"  ".repeat(2_usize.pow(3 - level as u32)));
            }
            result.push_str(&format!("{}", item));
            level_count += 1;

            if level_count == max_level_size {
                level += 1;
                level_count = 0;
                max_level_size *= 2;
            }
        }
        result
    }
}

impl<K: Ord, V> Heap<Entry<K, V>, DefaultComparator<Entry<K, V>>> {
    pub fn push_with_priority(&mut self, key: K, value: V) {
        self.push(Entry { key, value });
    }

    pub fn pop_with_priority(&mut self) -> Option<(K, V)> {
        self.pop().map(|entry| (entry.key, entry.value))
    }

    pub fn peek_with_priority(&self) -> Option<(&K, &V)> {
        self.peek().map(|entry| (&entry.key, &entry.value))
    }
}

impl<T: Ord> FromIterator<T> for Heap<T, DefaultComparator<T>> {
    fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self {
        let mut heap = Heap::new_min();
        for item in iter {
            heap.push(item);
        }
        heap
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Debug)]
    struct StrinLengthComparator;

    impl HeapComparator<String> for StrinLengthComparator {
        fn compare(&self, a: &String, b: &String) -> Ordering {
            a.len().cmp(&b.len())
        }
    }

    #[test]
    fn test_custom_comparator() {
        let mut heap = Heap::with_comparator(StrinLengthComparator, HeapType::Min);

        heap.push("hello".to_string());
        heap.push("a".to_string());
        heap.push("world".to_string());

        assert_eq!(heap.pop().unwrap(), "a");
        assert_eq!(heap.pop().unwrap().len(), 5)
    }

    #[test]
    fn test_viz() {
        let mut heap = Heap::new_min();
        heap.push(1);
        heap.push(2);
        heap.push(3);
        heap.push(4);
        heap.push(5);

        let viz = heap.visualize();
        assert!(!viz.is_empty());
        println!("Heap visualize:\n{}", viz);
    }
}
