use std::collections::HashMap;

pub fn flip_columns_for_max_no_equal_rows(matrix: Vec<Vec<i32>>) -> i32 {
    let mut pattern_freq: HashMap<String, i32> = HashMap::new();

    for row in matrix.iter() {
        if row.is_empty() {
            continue;
        }

        let first = row[0];
        let pattern: String = row
            .iter()
            .map(|&num| if num == first { 'T' } else { 'F' })
            .collect();
        *pattern_freq.entry(pattern).or_insert(0) += 1;
    }
    pattern_freq.values().max().copied().unwrap_or(0)
}
