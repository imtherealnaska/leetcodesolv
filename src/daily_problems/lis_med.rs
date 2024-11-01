pub fn length_of_lis(nums: Vec<i32>) -> i32 {
    if nums.is_empty() {
        return 0;
    }

    let mut sub = vec![nums[0]];

    for &num in nums.iter().skip(1) {
        if num > *sub.last().unwrap() {
            sub.push(num);
        } else {
            match sub.binary_search(&num) {
                Ok(pos) | Err(pos) => sub[pos] = num,
            }
        }
    }
    sub.len() as i32
}
