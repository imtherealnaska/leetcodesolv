pub fn min_subarray(nums: Vec<i32>, p: i32) -> i32 {
    let total_sum: i64 = nums.iter().map(|&x| x as i64).sum();
    let target = (total_sum % p as i64) as i32;

    if target == 0 {
        return 0;
    }

    let mut prefix_sum = 0;
    let mut seen = std::collections::HashMap::new();
    seen.insert(0, -1);
    let mut min_len = nums.len() as i32;

    for (i, &num) in nums.iter().enumerate() {
        prefix_sum = ((prefix_sum as i64 + num as i64) % p as i64) as i32;
        let complement = ((prefix_sum as i64 - target as i64 + p as i64) as i64) as i32;

        if let Some(&j) = seen.get(&complement) {
            min_len = min_len.min(i as i32 - j);
        }

        seen.insert(prefix_sum, i as i32);
    }

    if min_len == nums.len() as i32 {
        -1
    } else {
        min_len
    }
}
