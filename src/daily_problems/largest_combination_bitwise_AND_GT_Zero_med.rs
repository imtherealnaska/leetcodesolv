pub fn largest_combination(candidates: Vec<i32>) -> i32 {
    let mut max_count = 0;
    for i in 0..=23 {
        let mut count = 0;
        for num in &candidates {
            if num & (1 << i) != 0 {
                count += 1;
            }
        }
        max_count = max_count.max(count);
    }
    max_count
}
