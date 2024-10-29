use std::collections::HashSet;

pub fn longest_square_streak(nums: Vec<i32>) -> i32 {
    let mut uniq_nums: Vec<i32> = nums
        .into_iter()
        .collect::<HashSet<_>>()
        .into_iter()
        .collect();

    uniq_nums.sort_unstable();

    let mut longest_streak = 1;
    let num_set: HashSet<i32> = uniq_nums.iter().cloned().collect();

    for &num in &uniq_nums {
        let mut current = num;
        let mut streak = 1;

        while let Some(next) = current.checked_mul(current) {
            if num_set.contains(&next) {
                streak += 1;
                current = next;
            } else {
                break;
            }
        }
        longest_streak = longest_streak.max(streak);
    }
    if longest_streak >= 2 {
        longest_streak
    } else {
        -1
    }
}
