pub fn max_score_sightseeing_pair(values: Vec<i32>) -> i32 {
    let mut max_score = i32::MIN;

    let mut best_i_score = values[0] + 0;
    for j in 1..values.len() {
        let current_score = best_i_score + values[j] - j as i32;
        max_score = max_score.max(current_score);
        best_i_score = best_i_score.max(values[j] + j as i32);
    }
    max_score
}

#[test]
fn seeing_works() {
    assert_eq!(max_score_sightseeing_pair(vec![8, 1, 5, 2, 6]), 11);

    // Test case 2: [1,2]
    assert_eq!(max_score_sightseeing_pair(vec![1, 2]), 2);

    // Test case 4: [10,4,6,4,10]
    assert_eq!(max_score_sightseeing_pair(vec![10, 4, 6, 4, 10]), 16);
}
