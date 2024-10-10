pub fn max_width_ramp(nums: Vec<i32>) -> i32 {
    let n = nums.len();
    if n <= 1 {
        return 0;
    }

    // Create a stack to store indices of potentially useful elements
    let mut stack: Vec<usize> = Vec::new();

    // Push the first index and all indices with smaller values
    stack.push(0);
    for i in 1..n {
        if nums[i] < nums[*stack.last().unwrap()] {
            stack.push(i);
        }
    }

    let mut max_width = 0;

    // Iterate from right to left
    for i in (0..n).rev() {
        // Pop indices from stack while we have a valid ramp
        while !stack.is_empty() && nums[i] >= nums[*stack.last().unwrap()] {
            let j = stack.pop().unwrap();
            max_width = max_width.max(i - j);
        }

        // If stack is empty, we've found the maximum possible width
        if stack.is_empty() {
            break;
        }
    }

    max_width as i32
}

#[test]
fn it_works() {
    let nums = vec![9, 8, 1, 0, 1, 9, 4, 0, 4, 1];
    println!("{}", max_width_ramp(nums));
}
