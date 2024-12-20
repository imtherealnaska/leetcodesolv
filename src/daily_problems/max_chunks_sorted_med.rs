pub fn max_chunks_sorted(arr: Vec<i32>) -> i32 {
    let mut chunks = 0;
    let mut curr_max = 0;

    (0..arr.len()).for_each(|i| {
        curr_max = curr_max.max(arr[i]);
        if curr_max == i as i32 {
            chunks += 1;
        }
    });
    chunks
}
