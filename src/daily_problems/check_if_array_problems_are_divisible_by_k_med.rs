use std::collections::HashMap;

pub struct Solution;

impl Solution {
    pub fn can_arrange(arr: Vec<i32>, k: i32) -> bool {
        let n = arr.len();

        if n % 2 != 0 {
            return false;
        }

        let mut remainder_count = HashMap::new();

        for num in arr {
            let remainder = ((num % k) + k) % k;
            *remainder_count.entry(remainder).or_insert(0) += 1;
        }

        for (remainder, count) in remainder_count.iter() {
            if *remainder == 0 {
                if count % 2 != 0 {
                    return false;
                }
            } else {
                let complement = (k - remainder) % k;
                if remainder_count.get(remainder) != remainder_count.get(&complement) {
                    return false;
                }
            }
        }
        true
    }
}

#[test]
fn itworks() {
    let arr = vec![1, 2, 3, 4, 5, 10, 6, 7, 8, 9];
    let k = 5;
    assert!(Solution::can_arrange(arr, k));
}

#[test]
fn itfails() {
    let arr = vec![1, 2, 3, 4, 5, 6];
    let k = 10;
    assert!(!Solution::can_arrange(arr, k));
}
