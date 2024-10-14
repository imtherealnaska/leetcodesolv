use std::collections::BinaryHeap;

pub fn max_klements(nums: Vec<i32>, k: i32) -> i64 {
    //    let mut heap = BinaryHeap::new();
    //
    //    for &num in nums {
    //        heap.push(num);
    //    }
    let mut heap = BinaryHeap::from(nums);

    let mut score = 0i64;

    for _ in 0..k {
        if let Some(max_val) = heap.pop() {
            score += max_val as i64;
            let new_val = (max_val + 2) / 3;
            heap.push(new_val);
        }
    }
    score
}
