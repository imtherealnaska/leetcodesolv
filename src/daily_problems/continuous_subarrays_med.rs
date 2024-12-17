pub fn continuous_subarrays_med(nums: Vec<i32>) -> i64 {
    use std::collections::BTreeMap;

    let mut left = 0;
    let mut count = 0_i64;
    let mut freq_map = BTreeMap::new();

    for right in 0..nums.len() {
        *freq_map.entry(nums[right]).or_insert(0) += 1;

        while freq_map.keys().next_back().unwrap() - freq_map.keys().next().unwrap() > 2 {
            if let Some(entry) = freq_map.get_mut(&nums[left]) {
                *entry -= 1;
                if *entry == 0 {
                    freq_map.remove(&nums[left]);
                }
            }
            left += 1;
        }
        count += (right - left + 1) as i64;
    }
    count
}
