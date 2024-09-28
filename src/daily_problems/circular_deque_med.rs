#[derive(Debug)]
struct MyCircularDeque {
    data: Vec<i32>,
    front: usize,
    rear: usize,
    size: usize,
    capacity: usize,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl MyCircularDeque {
    fn new(k: i32) -> Self {
        Self {
            data: vec![0; k as usize],
            front: 0,
            rear: 0,
            size: 0,
            capacity: k as usize,
        }
    }

    fn insert_front(&mut self, value: i32) -> bool {
        if self.is_full() {
            return false;
        }
        if !self.is_empty() {
            self.front = (self.front + self.capacity - 1) % self.capacity;
        }

        self.data[self.front] = value;
        self.size += 1;
        true
    }

    fn insert_last(&mut self, value: i32) -> bool {
        if self.is_full() {
            return false;
        }

        if !self.is_empty() {
            self.rear = (self.rear + 1) % self.capacity;
        }

        self.data[self.rear] = value;
        self.size += 1;
        true
    }

    fn delete_front(&mut self) -> bool {
        if self.is_empty() {
            return false;
        }

        if self.size > 1 {
            self.front = (self.front + 1) % self.capacity;
        }
        self.size -= 1;
        true
    }

    fn delete_last(&mut self) -> bool {
        if self.is_empty() {
            return false;
        }
        if self.size > 1 {
            self.rear = (self.rear + self.capacity - 1) % self.capacity;
        }
        self.size -= 1;
        true
    }

    fn get_front(&self) -> i32 {
        if self.is_empty() {
            -1
        } else {
            self.data[self.front]
        }
    }

    fn get_rear(&self) -> i32 {
        if self.is_empty() {
            -1
        } else {
            self.data[self.rear]
        }
    }

    fn is_empty(&self) -> bool {
        self.size == 0
    }

    fn is_full(&self) -> bool {
        self.size == self.capacity
    }
}

#[test]
fn circular_deque_med_works() {
    let k = 5;
    let value = 24;
    let mut obj = MyCircularDeque::new(k);
    println!("{:?}", obj);
    let ret_1: bool = obj.insert_front(value);
    println!("{:?}", obj);
    let ret_2: bool = obj.insert_last(value);
    println!("{:?}", obj);
    let ret_3: bool = obj.delete_front();
    println!("{:?}", obj);
    let ret_4: bool = obj.delete_last();
    println!("{:?}", obj);
    let ret_5: i32 = obj.get_front();
    println!("{:?}", obj);
    let ret_6: i32 = obj.get_rear();
    println!("{:?}", obj);
    let ret_7: bool = obj.is_empty();
    println!("{:?}", obj);
    let ret_8: bool = obj.is_full();
    println!("{:?}", obj);
}
