pub fn find_target_sum_ways(nums: Vec<i32>, target: i32) -> i32 {
    let sum: i32 = nums.iter().sum();

    if target.abs() > sum {
        return 0;
    }

    let shift = sum;
    let total_range = 2 * sum + 1;
    let mut dp = vec![vec![0; total_range as usize]; nums.len() + 1];
    dp[0][shift as usize] = 1;

    for i in 0..nums.len() {
        for j in 0..total_range {
            if dp[i][j as usize] > 0 {
                if j + nums[i] < total_range {
                    dp[i + 1][(j + nums[i]) as usize] += dp[i][j as usize];
                }

                if j - nums[i] >= 0 {
                    dp[i + 1][(j - nums[i]) as usize] += dp[i][j as usize];
                }
            }
        }
    }
    dp[nums.len()][(target + shift) as usize]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_find_target_sum_ways() {
        // Test case 1: nums = [1,1,1,1,1], target = 3
        assert_eq!(find_target_sum_ways(vec![1, 1, 1, 1, 1], 3), 5);

        // Test case 2: nums = [1], target = 1
        assert_eq!(find_target_sum_ways(vec![1], 1), 1);

        // Test case 3: Empty array
        assert_eq!(find_target_sum_ways(vec![], 0), 1);

        // Test case 4: Impossible target
        assert_eq!(find_target_sum_ways(vec![1, 1], 4), 0);
    }
}
