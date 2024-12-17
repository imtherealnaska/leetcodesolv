pub fn maximum_beauty(nums: Vec<i32>, k: i32) -> i32 {
    let mut sorted_nums = nums.clone();
    sorted_nums.sort_unstable();

    let n = sorted_nums.len();
    let mut max_beauty = 1;

    let mut left = 0;
    for right in 0..n {
        while sorted_nums[right] - sorted_nums[left] > 2 * k {
            left += 1;
        }
        max_beauty = max_beauty.max(right - left + 1);
    }
    max_beauty as i32
}
