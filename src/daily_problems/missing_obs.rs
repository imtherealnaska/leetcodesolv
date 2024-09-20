pub fn missing_rolls(rolls: Vec<i32>, mean: i32, n: i32) -> Vec<i32> {
    let m = rolls.len() as i32;
    let total_sum = mean * (n + m);
    let missing_sum = total_sum - rolls.iter().sum::<i32>();

    if missing_sum < n || missing_sum > 6 * n {
        return vec![];
    }

    let mut result = vec![missing_sum / n; n as usize];
    let remainder = missing_sum % n;

    (0..remainder as usize).for_each(|i| {
        result[i] += 1;
    });
    result
}
