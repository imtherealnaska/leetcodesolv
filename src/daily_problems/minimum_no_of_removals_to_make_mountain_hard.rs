use std::cmp::Reverse;

pub fn minimum_mountain_removals(nums: Vec<i32>) -> i32 {
    let n = nums.len();
    if n < 3 {
        return 0;
    }

    let mut lis = vec![1; n];
    for i in 1..n {
        for j in 0..i {
            if nums[i] > nums[j] {
                lis[i] = lis[i].max(lis[j] + 1);
            }
        }
    }

    let mut lds = vec![1; n];
    for i in (0..n - 1).rev() {
        for j in (i + 1)..n {
            if nums[i] > nums[j] {
                lds[i] = lds[i].max(lds[j] + 1);
            }
        }
    }

    let mut max_moutain_len = 0;
    for i in 1..n - 1 {
        if lis[i] > 1 && lds[i] > 1 {
            max_moutain_len = max_moutain_len.max(lis[i] + lds[i] - 1);
        }
    }

    if max_moutain_len >= 2 {
        (n as i32) - max_moutain_len
    } else {
        n as i32 - 3
    }
}

pub fn using_bs(nums: Vec<i32>) -> i32 {
    let n = nums.len();
    if n < 3 {
        return 0;
    }

    // Step 1: Compute LIS for each index with binary search
    let mut lis = vec![1; n];
    let mut increasing_seq = Vec::new();
    for i in 0..n {
        let pos = increasing_seq.binary_search(&nums[i]).unwrap_or_else(|x| x);
        if pos >= increasing_seq.len() {
            increasing_seq.push(nums[i]);
        } else {
            increasing_seq[pos] = nums[i];
        }
        lis[i] = pos as i32 + 1;
    }

    // Step 2: Compute LDS for each index with binary search
    let mut lds = vec![1; n];
    let mut decreasing_seq = Vec::new();
    for i in (0..n).rev() {
        let pos = decreasing_seq
            .binary_search(&Reverse(nums[i]))
            .unwrap_or_else(|x| x);
        if pos >= decreasing_seq.len() {
            decreasing_seq.push(Reverse(nums[i]));
        } else {
            decreasing_seq[pos] = Reverse(nums[i]);
        }
        lds[i] = pos as i32 + 1;
    }

    // Step 3: Find the longest mountain array
    let mut max_mountain_len = 0;
    for i in 1..n - 1 {
        if lis[i] > 1 && lds[i] > 1 {
            max_mountain_len = max_mountain_len.max(lis[i] + lds[i] - 1);
        }
    }

    // Step 4: Calculate the minimum number of elements to remove
    if max_mountain_len >= 3 {
        (n as i32) - max_mountain_len
    } else {
        n as i32 - 3
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn bs_works() {
        let nums = vec![2, 1, 1, 5, 6, 2, 3, 1];
        assert_eq!(3, using_bs(nums));
    }

    #[test]
    fn normal_works() {
        let nums = vec![2, 1, 1, 5, 6, 2, 3, 1];
        assert_eq!(3, minimum_mountain_removals(nums));
    }
}
