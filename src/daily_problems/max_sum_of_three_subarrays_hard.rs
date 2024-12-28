pub fn max_sum_of_three_subarrays(nums: Vec<i32>, k: i32) -> Vec<i32> {
    let k = k as usize;

    let mut best_single_start = 0;
    let mut best_double_start = vec![0, k];
    let mut best_triple_start = vec![0, k, k * 2];

    let mut current_window_sum_single: i32 = nums[..k].iter().sum();
    let mut current_window_sum_duble: i32 = nums[k..k * 2].iter().sum();
    let mut current_window_sum_triple: i32 = nums[k * 2..k * 3].iter().sum();

    let mut best_single_sum = current_window_sum_single;
    let mut best_double_sum = current_window_sum_duble;
    let mut best_triple_sum = current_window_sum_triple;

    let mut single_start_index = 1;
    let mut double_start_index = k + 1;
    let mut triple_start_index = k * 2 + 1;

    while triple_start_index <= nums.len() - k {
        current_window_sum_single = current_window_sum_single - nums[single_start_index - 1]
            + nums[single_start_index + k - 1];
        current_window_sum_duble = current_window_sum_duble - nums[double_start_index - 1]
            + nums[double_start_index + k - 1];
        current_window_sum_triple = current_window_sum_triple - nums[triple_start_index - 1]
            + nums[triple_start_index + k - 1];

        if current_window_sum_single > best_single_sum {
            best_single_start = single_start_index;
            best_single_sum = current_window_sum_single;
        }

        // Update the best double subarray start indices if a better sum is found
        if current_window_sum_duble + best_single_sum > best_double_sum {
            best_double_start[0] = best_single_start;
            best_double_start[1] = double_start_index;
            best_double_sum = current_window_sum_duble + best_single_sum;
        }

        // Update the best triple subarray start indices if a better sum is found
        if current_window_sum_triple + best_double_sum > best_triple_sum {
            best_triple_start[0] = best_double_start[0];
            best_triple_start[1] = best_double_start[1];
            best_triple_start[2] = triple_start_index;
            best_triple_sum = current_window_sum_triple + best_double_sum;
        }

        // Move the sliding windows forward
        single_start_index += 1;
        double_start_index += 1;
        triple_start_index += 1;
    }
    best_triple_start.iter().map(|&x| x as i32).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_max_sum_of_three_subarrays() {
        let nums = vec![1, 2, 1, 2, 6, 7, 5, 1];
        let k = 2;
        assert_eq!(max_sum_of_three_subarrays(nums, k), vec![0, 3, 5]);

        let nums = vec![1, 2, 1, 2, 1, 2, 1, 2, 1];
        let k = 2;
        assert_eq!(max_sum_of_three_subarrays(nums, k), vec![0, 2, 4]);
    }
}
