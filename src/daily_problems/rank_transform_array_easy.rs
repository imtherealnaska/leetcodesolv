use std::collections::HashMap;

struct Solution;

impl Solution {
    pub fn array_rank_transform(arr: Vec<i32>) -> Vec<i32> {
        if arr.is_empty() {
            return vec![];
        }

        let mut sorted = arr.clone();
        sorted.sort_unstable();

        let mut rankmap = HashMap::new();
        let mut current_rank = 1;

        for &num in &sorted {
            rankmap.entry(num).or_insert_with(|| {
                let rank = current_rank;
                current_rank += 1;
                rank
            });
        }

        arr.iter().map(|&num| rankmap[&num]).collect()
    }
}
