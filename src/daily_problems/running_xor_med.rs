pub fn get_maximum_xor(nums: Vec<i32>, maximum_bit: i32) -> Vec<i32> {
    let n = nums.len();
    let mut answer = vec![0; n];
    let mut max_value = (1 << maximum_bit) - 1;
    let mut running_xor = 0;
    for &num in &nums {
        running_xor ^= num;
    }

    (0..n).rev().for_each(|i| {
        answer[n - 1 - i] = max_value ^ running_xor;

        if i > 0 {
            running_xor ^= nums[i];
        }
    });
    answer
}

#[test]
fn xorrring_works() {
    let nums = vec![0, 1, 1, 3];
    let maximum_bit = 2;
    let result = get_maximum_xor_value(nums, maximum_bit);
    println!("{:?}", result); // [0,3,2,3]

    let nums = vec![2, 3, 4, 7];
    let maximum_bit = 3;
    let result = get_maximum_xor_value(nums, maximum_bit);
    println!("{:?}", result); // [5,2,6,5]
}
