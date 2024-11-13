pub fn count_fair_pairs(nums: Vec<i32>, lower: i32, upper: i32) -> i64 {
    let mut sorted_nums = nums.clone();
    sorted_nums.sort();
    let n = sorted_nums.len();

    let count_pairs = |target: i32| -> i64 {
        let mut count: i64 = 0;
        let mut left = 0;
        let mut right = n - 1;

        while left < right {
            let sum = sorted_nums[left] + sorted_nums[right];
            if sum <= target {
                count += (right - left) as i64;
                left += 1;
            } else {
                right -= 1;
            }
        }
        count
    };
    count_pairs(upper) - count_pairs(lower - 1)
}
