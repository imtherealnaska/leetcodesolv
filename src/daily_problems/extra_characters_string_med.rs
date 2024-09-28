// You are given a 0-indexed string s and a dictionary of words dictionary.
// You have to break s into one or more non-overlapping substrings such that each substring is present in dictionary.
// There may be some extra characters in s which are not present in any of the substrings.
// Return the minimum number of extra characters left over if you break up s optimally.

use std::collections::HashSet;

struct Solution;
impl Solution {
    pub fn min_extra_char(s: String, dictionary: Vec<String>) -> i32 {
        let n = s.len();
        let dict: HashSet<String> = dictionary.into_iter().collect();
        let mut dp = vec![0; n + 1];

        for i in 1..=n {
            dp[i] = dp[i - 1] + 1;
            for j in 0..i {
                if dict.contains(&s[j..i]) {
                    dp[i] = dp[i].min(dp[j]);
                }
            }
        }
        dp[n]
    }
}
