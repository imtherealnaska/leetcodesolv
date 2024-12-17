pub fn min_val_replacement_easy(nums: Vec<i32>, k: i32, multiplier: i32) -> Vec<i32> {
    let mut result = nums.clone();

    for _ in 0..k {
        let min_index = result
            .iter()
            .enumerate()
            .min_by_key(|&(_, &val)| val)
            .map(|(index, _)| index)
            .unwrap_or(0);

        result[min_index] *= multiplier;
    }
    result
}

#[test]
fn test_1() {
    let nums = vec![3, 1, 4, 2, 5];
    let k = 3;
    let multiplier = 2;

    let result = min_val_replacement_easy(nums, k, multiplier);
    println!("Result {:?}", result);
}
