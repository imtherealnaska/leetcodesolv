#[derive(Debug, Clone, Copy, PartialEq)]
pub enum HeapType {
    Min,
    Max,
}

#[derive(Debug)]
pub struct Heap<T: Ord> {
    items: Vec<T>,
    heap_type: HeapType,
}

impl<T: Ord> Default for Heap<T> {
    fn default() -> Self {
        Self::new_min()
    }
}

impl<T: Ord> Heap<T> {
    pub fn new_min() -> Self {
        Self {
            items: Vec::new(),
            heap_type: HeapType::Min,
        }
    }

    pub fn new_max() -> Self {
        Self {
            items: Vec::new(),
            heap_type: HeapType::Max,
        }
    }

    pub fn with_capacity(capacity: usize, heap_type: HeapType) -> Self {
        Self {
            items: Vec::with_capacity(capacity),
            heap_type,
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

    pub fn decrease_key(&mut self, index: usize, new_value: T) -> Result<(), &'static str> {
        if index >= self.len() {
            return Err("Index out of bounds");
        }

        match self.heap_type {
            HeapType::Min => {
                if new_value >= self.items[index] {
                    return Err("New value must be smaller for min-heap");
                }
            }
            HeapType::Max => {
                if new_value <= self.items[index] {
                    return Err("New value must be larger for max-heap");
                }
            }
        }
        self.items[index] = new_value;
        self.sift_up(index);
        Ok(())
    }

    pub fn compare(&self, a: &T, b: &T) -> bool {
        match self.heap_type {
            HeapType::Min => a < b,
            HeapType::Max => a > b,
        }
    }

    fn parent(index: usize) -> usize {
        (index - 1) / 2
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
            if self.compare(&self.items[index], &self.items[parent]) {
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

            if left < self.len() && self.compare(&self.items[left], &self.items[target]) {
                target = left;
            }

            if right < self.len() && self.compare(&self.items[right], &self.items[target]) {
                target = right;
            }

            if target == index {
                break;
            }

            self.items.swap(index, target);
            index = target;
        }
    }

    pub fn merge(&mut self, other: Heap<T>) {
        if self.heap_type != other.heap_type {
            panic!("Cannot merge heaps of different type");
        }

        for item in other.items {
            self.push(item);
        }
    }
}

impl<T: Ord> FromIterator<T> for Heap<T> {
    fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self {
        let mut heap = Heap::new_min();
        for item in iter {
            heap.push(item);
        }
        heap
    }
}

impl<T: Ord> IntoIterator for Heap<T> {
    type Item = T;

    type IntoIter = std::vec::IntoIter<T>;

    fn into_iter(self) -> Self::IntoIter {
        self.items.into_iter()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_min_heap() {
        let mut heap = Heap::new_min();
        heap.push(3);
        heap.push(1);
        heap.push(4);
        heap.push(2);

        assert_eq!(heap.pop(), Some(1));
        assert_eq!(heap.pop(), Some(2));
        assert_eq!(heap.pop(), Some(3));
        assert_eq!(heap.pop(), Some(4));
    }

    #[test]
    fn test_max_heap() {
        let mut heap = Heap::new_max();
        heap.push(3);
        heap.push(1);
        heap.push(4);
        heap.push(2);

        assert_eq!(heap.pop(), Some(4));
        assert_eq!(heap.pop(), Some(3));
        assert_eq!(heap.pop(), Some(2));
        assert_eq!(heap.pop(), Some(1));
    }

    #[test]
    fn test_from_iterator() {
        let vec = vec![3, 1, 4, 1, 5, 9, 2, 6, 5, 3, 5];
        let mut heap: Heap<_> = vec.into_iter().collect();

        let mut sorted = Vec::new();
        while let Some(item) = heap.pop() {
            sorted.push(item);
        }

        assert!(sorted.windows(2).all(|w| w[0] <= w[1]));
    }

    #[test]
    fn test_decrease_key() {
        let mut heap = Heap::new_min();
        heap.push(5);
        heap.push(4);
        heap.push(3);

        assert!(heap.decrease_key(1, 1).is_ok());
        assert_eq!(heap.pop(), Some(1));
    }

    #[test]
    fn test_merge() {
        let mut heap1 = Heap::new_min();
        heap1.push(3);
        heap1.push(1);

        let mut heap2 = Heap::new_min();
        heap2.push(4);
        heap2.push(2);

        heap1.merge(heap2);

        assert_eq!(heap1.len(), 4);
        assert_eq!(heap1.pop(), Some(1));
        assert_eq!(heap1.pop(), Some(2));
        assert_eq!(heap1.pop(), Some(3));
        assert_eq!(heap1.pop(), Some(4));
    }
}
