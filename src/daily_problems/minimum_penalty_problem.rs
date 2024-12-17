pub fn minimum_penalty(nums: Vec<i32>, max_operations: i32) -> i32 {
    let mut left = 1;
    let mut right = *nums.iter().max().unwrap();

    while left < right {
        let mid = left + (right - left) / 2;

        if can_divide_bags(mid, &nums, max_operations) {
            right = mid;
        } else {
            left = mid + 1;
        }
    }
    left
}

fn can_divide_bags(target: i32, nums: &[i32], max_operations: i32) -> bool {
    let mut operations_needed = 0;

    for &num in nums {
        operations_needed += divide_bag_count(num, target);
    }
    operations_needed <= max_operations
}

fn divide_bag_count(bag_size: i32, target: i32) -> i32 {
    if bag_size <= target {
        return 0;
    }

    (bag_size - 1) / target
}
