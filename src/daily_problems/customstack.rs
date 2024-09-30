/**
 * Your CustomStack object will be instantiated and called as such:
 * let obj = CustomStack::new(maxSize);
 * obj.push(x);
 * let ret_2: i32 = obj.pop();
 * obj.increment(k, val);
 */
#[derive(Debug)]
struct CustomStack {
    stack: Vec<i32>,
    max_size: usize,
}

trait NewTrait {
    fn new(max_size: i32) -> Self;

    fn push(&mut self, x: i32);

    fn pop(&mut self) -> i32;

    fn increment(&mut self, k: i32, val: i32);
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl NewTrait for CustomStack {
    fn new(max_size: i32) -> Self {
        CustomStack {
            stack: Vec::with_capacity(max_size as usize),
            max_size: max_size as usize,
        }
    }

    fn push(&mut self, x: i32) {
        if self.stack.len() < self.max_size {
            self.stack.push(x);
        }
    }

    fn pop(&mut self) -> i32 {
        self.stack.pop().unwrap_or(-1)
    }

    fn increment(&mut self, k: i32, val: i32) {
        let k = k as usize;
        let limit = k.min(self.stack.len());
        for i in 0..limit {
            self.stack[i] += val;
        }
    }
}

#[test]
fn works() {
    let max_size = 100;
    let x = 100;
    let k = 50;
    let val = 55;
    let mut obj = CustomStack::new(max_size);
    println!("{:?}", obj);
    obj.push(x);
    println!("{:?}", obj);
    let ret_2: i32 = obj.pop();
    println!("{ret_2}");
    obj.increment(k, val);
    println!("{:?}", obj);
}
