pub fn prime_sub_operation(nums: Vec<i32>) -> bool {
    let max_element = *nums.iter().max().unwrap() as usize;

    // Create and initialize sieve array
    let mut sieve = vec![1; max_element + 1];
    if max_element >= 1 {
        sieve[1] = 0;
    }

    // Calculate primes using sieve of Eratosthenes
    let sqrt_max = (f64::sqrt((max_element + 1) as f64) as usize).min(max_element);
    for i in 2..=sqrt_max {
        if sieve[i] == 1 {
            let mut j = i * i;
            while j <= max_element {
                sieve[j] = 0;
                j += i;
            }
        }
    }

    let mut curr_value = 1;
    let mut i = 0;

    while i < nums.len() {
        let difference = nums[i] - curr_value;

        if difference < 0 {
            return false;
        }

        if difference == 0
            || (difference as usize <= max_element && sieve[difference as usize] == 1)
        {
            i += 1;
            curr_value += 1;
        } else {
            curr_value += 1;
        }
    }

    true
}
