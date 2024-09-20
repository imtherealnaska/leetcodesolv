pub fn longest_subarray(nums: Vec<i32>) -> i32 {
    let max_val = *nums.iter().max().unwrap();

    let mut longest = 0;
    let mut current_length = 0;

    // Iterate through the array once
    for &num in nums.iter() {
        // If the current number is equal to the maximum value, extend the current subarray
        if num == max_val {
            current_length += 1;
            // Update the longest length if necessary
            longest = longest.max(current_length);
        } else {
            // Reset current length if we encounter a number not equal to max_val
            current_length = 0;
        }
    }

    longest
}
