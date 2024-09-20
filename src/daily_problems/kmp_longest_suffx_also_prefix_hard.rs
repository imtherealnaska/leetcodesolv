pub struct Solution;

impl Solution {
    pub fn shortest_palindrome(s: String) -> String {
        let rev_s: String = s.chars().rev().collect();
        let new_s = format!("{}#{}", s, rev_s);
        let lps = Self::compute_kmp_table(&new_s);
        let add_len = s.len() - lps[new_s.len() - 1];
        let to_add: String = s.chars().rev().take(add_len).collect();
        format!("{}{}", to_add, s)
    }

    pub fn compute_kmp_table(s: &str) -> Vec<usize> {
        let n = s.len();
        let mut lps = vec![0; n];
        let chars: Vec<char> = s.chars().collect();
        let mut len = 0;
        let mut i = 1;

        while i < n {
            if chars[i] == chars[len] {
                len += 1;
                lps[i] = len;
                i += 1;
            } else if len != 0 {
                len = lps[len - 1];
            } else {
                lps[i] = 0;
                i += 1;
            }
        }
        lps
    }
}

#[test]
fn test_palindrom() {
    let s = String::from("aacecaaa");
    let result = Solution::shortest_palindrome(s);
    println!("{}", result); // Output: "aaacecaaa"
}
