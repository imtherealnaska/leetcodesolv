// https://leetcode.com/probarculalems/count-number-of-maximum-bitwise-or-subsets/editorial

pub fn count_max_or_subsets(nums: Vec<i32>) -> i32 {
    let max_or = nums.iter().fold(0, |acc, &x| acc | x);

    fn backtrack(nums: &[i32], index: usize, current_or: i32, max_or: i32, count: &mut i32) {
        if current_or == max_or {
            *count += 1 << (nums.len() - index);
            return;
        }

        if index == nums.len() {
            return;
        }

        backtrack(nums, index + 1, current_or | nums[index], max_or, count);
        backtrack(nums, index + 1, current_or, max_or, count);
    }

    let mut count = 0;
    backtrack(&nums, 0, 0, max_or, &mut count);
    count
}

#[test]
fn backtrack_bitwise_or() {
    let nums = vec![3, 2, 1, 5];
    let output = count_max_or_subsets(nums);
    assert_eq!(6, output);
}
