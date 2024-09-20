pub fn xor_queries(arr: Vec<i32>, queries: Vec<Vec<i32>>) -> Vec<i32> {
    let mut prefix_xor = vec![0; arr.len() + 1];
    for i in 0..arr.len() {
        prefix_xor[i + 1] = prefix_xor[i] ^ arr[i];
    }

    queries
        .into_iter()
        .map(|query| {
            let left = query[0] as usize;
            let right = query[1] as usize;
            prefix_xor[right + 1] ^ prefix_xor[left]
        })
        .collect()
}
