use std::collections::BinaryHeap;

pub fn pick_gifts(gifts: Vec<i32>, k: i32) -> i64 {
    let mut heap: BinaryHeap<i32> = gifts.into_iter().collect();
    for _ in 0..k {
        if let Some(max_gift) = heap.pop() {
            heap.push((max_gift as f64).sqrt().floor() as i32);
        }
    }
    heap.iter().map(|&x| x as i64).sum()
}
