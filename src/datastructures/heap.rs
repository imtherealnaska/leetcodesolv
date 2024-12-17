#[derive(Debug)]
pub struct MinHeap<T: Ord> {
    items: Vec<T>,
}

impl<T: Ord> MinHeap<T> {
    pub fn new() -> Self {
        MinHeap { items: Vec::new() }
    }

    pub fn with_capacity(capacity: usize) -> Self {
        MinHeap {
            items: Vec::with_capacity(capacity),
        }
    }

    pub fn len(&self) -> usize {
        self.items.len()
    }

    pub fn is_empty(&self) -> bool {
        self.items.is_empty()
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
            if self.items[index] < self.items[parent] {
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

            let mut smallest = index;

            if left < self.len() && self.items[left] < self.items[smallest] {
                smallest = left;
            }

            if right < self.len() && self.items[right] < self.items[smallest] {
                smallest = right;
            }

            if smallest == index {
                break;
            }

            self.items.swap(index, smallest);
            index = smallest;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_min_heap_push_and_peek() {
        let mut heap = MinHeap::new();
        heap.push(10);
        heap.push(5);
        heap.push(20);

        // The smallest element should be at the top
        assert_eq!(heap.peek(), Some(&5));
    }

    #[test]
    fn test_min_heap_pop() {
        let mut heap = MinHeap::new();
        heap.push(10);
        heap.push(5);
        heap.push(20);

        // Pop should return elements in ascending order
        assert_eq!(heap.pop(), Some(5));
        assert_eq!(heap.pop(), Some(10));
        assert_eq!(heap.pop(), Some(20));
        assert_eq!(heap.pop(), None); // Empty heap
    }

    #[test]
    fn test_min_heap_is_empty() {
        let mut heap = MinHeap::new();
        assert!(heap.is_empty());

        heap.push(1);
        assert!(!heap.is_empty());

        heap.pop();
        assert!(heap.is_empty());
    }

    #[test]
    fn test_min_heap_with_duplicates() {
        let mut heap = MinHeap::new();
        heap.push(10);
        heap.push(10);
        heap.push(5);
        heap.push(20);

        assert_eq!(heap.pop(), Some(5));
        assert_eq!(heap.pop(), Some(10));
        assert_eq!(heap.pop(), Some(10));
        assert_eq!(heap.pop(), Some(20));
    }

    #[test]
    fn test_min_heap_large_input() {
        let mut heap = MinHeap::new();
        for i in (1..=1000).rev() {
            heap.push(i);
        }

        for i in 1..=1000 {
            assert_eq!(heap.pop(), Some(i));
        }

        assert!(heap.is_empty());
    }

    #[test]
    fn test_min_heap_custom_type() {
        #[derive(Debug, Eq, PartialEq, Ord, PartialOrd)]
        struct CustomType {
            priority: i32,
        }

        let mut heap = MinHeap::new();
        heap.push(CustomType { priority: 10 });
        heap.push(CustomType { priority: 5 });
        heap.push(CustomType { priority: 20 });

        assert_eq!(heap.pop(), Some(CustomType { priority: 5 }));
        assert_eq!(heap.pop(), Some(CustomType { priority: 10 }));
        assert_eq!(heap.pop(), Some(CustomType { priority: 20 }));
    }

    #[test]
    fn test_min_heap_empty_peek_and_pop() {
        let mut heap: MinHeap<i32> = MinHeap::new();
        assert_eq!(heap.peek(), None);
        assert_eq!(heap.pop(), None);
    }
}
