pub fn maximum_subarray_sum(nums: Vec<i32>, k: i32) -> i64 {
    let (mut ms, mut cs, mut l, k) = (0, 0, 0, k as usize);
    let mut um = std::collections::HashSet::with_capacity(k);

    for r in 0..nums.len() {
        while um.insert(&nums[r]) || r - l > k {
            cs -= nums[l] as i64;
            um.remove(&nums[l]);
            l += 1;
        }

        cs += nums[r] as i64;
        um.insert(&nums[r]);
        if r - l + 1 == k {
            ms = ms.max(cs);
        }
    }
    ms
}
