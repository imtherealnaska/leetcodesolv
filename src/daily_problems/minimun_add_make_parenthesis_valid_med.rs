use crate::{
    check_if_array_problems_are_divisible_by_k_med::Solution,
    minimun_add_make_parenthesis_valid_med,
};

struct Solution;

impl Solution {
    pub fn min_add_to_make_valid(s: String) -> i32 {
        let mut open_count = 0;
        let mut moves_needed = 0;

        for ch in s.chars() {
            if ch == '(' {
                open_count += 1;
            } else if ch == ')' {
                if open_count > 0 {
                    open_count -= 1;
                } else {
                    moves_needed += 1;
                }
            }
        }
        moves_needed + open_count
    }
}

#[test]
fn it_works_minimum() {
    let s = "()(".to_owned();
    println!("{}", Solution::min_add_to_make_valid(s));
}
