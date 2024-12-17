use std::{cmp::Reverse, collections::BinaryHeap};

pub fn find_score(nums: Vec<i32>) -> i64 {
    let mut marked = vec![false; nums.len()];
    let mut heap = BinaryHeap::new();

    for (index, &number) in nums.iter().enumerate() {
        heap.push(Reverse((number, index)));
    }

    let mut ans: i64 = 0;

    while let Some(Reverse((number, index))) = heap.pop() {
        if !marked[index] {
            ans += number as i64;
            marked[index] = true;

            if index > 0 {
                marked[index - 1] = true;
            }

            if index + 1 < nums.len() {
                marked[index + 1] = true;
            }
        }
    }
    ans
}
