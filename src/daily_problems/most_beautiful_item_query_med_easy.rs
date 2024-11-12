pub fn maximum_beauty(mut items: Vec<Vec<i32>>, queries: Vec<i32>) -> Vec<i32> {
    let mut ans = vec![0; queries.len()];
    items.sort_by(|a, b| a[0].cmp(&b[0]));

    let mut queries_with_indices: Vec<Vec<i32>> = queries
        .iter()
        .enumerate()
        .map(|(i, &query)| vec![query, i as i32])
        .collect();

    queries_with_indices.sort_by(|a, b| a[0].cmp(&b[0]));

    let mut item_index = 0;
    let mut max_beauty = 0;

    for query_info in queries_with_indices.iter() {
        let query = query_info[0];
        let original_index = query_info[1] as usize;

        while item_index < items.len() && items[item_index][0] <= query {
            max_beauty = max_beauty.max(items[item_index][1]);
            item_index += 1;
        }
        ans[original_index] = max_beauty;
    }
    ans
}

#[test]
fn max_beauty_works() {
    let items = vec![vec![1, 2], vec![3, 2], vec![2, 4], vec![5, 6], vec![3, 5]];
    let queries = vec![1, 2, 3, 4, 5, 6];
    assert_eq!(vec![2, 4, 5, 5, 6, 6], maximum_beauty(items, queries));
}
