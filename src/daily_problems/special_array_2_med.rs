pub fn is_array_special(nums: Vec<i32>, queries: Vec<Vec<i32>>) -> Vec<bool> {
    let n = nums.len();
    let mut max_reach = vec![0; n];
    let mut end = 0;

    for start in 0..n {
        end = end.max(start);
        while end < n - 1 && nums[end] % 2 != nums[end + 1] % 2 {
            end += 1;
        }
        max_reach[start] = end;
    }

    queries
        .iter()
        .map(|query| {
            let start = query[0] as usize;
            let end_query = query[1] as usize;

            end_query <= max_reach[start]
        })
        .collect()
}
