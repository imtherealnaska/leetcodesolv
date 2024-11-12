pub fn minimum_subarray_length(nums: Vec<i32>, k: i32) -> i32 {
    let mut min_length = i32::MAX;
    let mut window_start = 0;
    let mut window_end = 0;

    let mut bit_counts = vec![0; 32];

    while window_end < nums.len() {
        update_bit_counts(&mut bit_counts, nums[window_end], 1);
        while window_start <= window_end && convert_bit_counts_to_number(&bit_counts) >= k {
            min_length = min_length.min((window_end - window_start + 1) as i32);
            update_bit_counts(&mut bit_counts, nums[window_start], -1);
            println!("start {}", window_start);
            window_start += 1;
            println!("start {}", window_start);
        }
        window_end += 1;
        println!("end : {}", window_end);
    }
    if min_length == i32::MAX {
        -1
    } else {
        min_length
    }
}

fn update_bit_counts(bit_counts: &mut [i32], number: i32, arg: i32) {
    (0..32).for_each(|bit_position| {
        if (number >> bit_position) & 1 != 0 {
            bit_counts[bit_position] += arg;
        }
    });
    println!("citcounts {bit_counts:?}");
}

fn convert_bit_counts_to_number(bit_counts: &[i32]) -> i32 {
    let mut result = 0;
    (0..32).for_each(|bit_pos| {
        if bit_counts[bit_pos] != 0 {
            result |= 1 << bit_pos;
        }
    });
    println!("result {result}");
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn minimum_subarray_length_works() {
        assert_eq!(minimum_subarray_length(vec![1, 2, 3], 2), 1);
        // assert_eq!(minimum_subarray_length(vec![2, 1, 8], 10), 3);
        // assert_eq!(minimum_subarray_length(vec![1, 2], 0), 1);
    }
}
