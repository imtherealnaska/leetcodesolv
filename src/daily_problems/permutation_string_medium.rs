use std::collections::HashMap;

// Example 1:
// Input: s1 = "ab", s2 = "eidbaooo"
// Output: true
// Explanation: s2 contains one permutation of s1 ("ba").
// Example 2:
// Input: s1 = "ab", s2 = "eidboaoo"
// Output: false
struct Solution;

trait NewTrait {
    // Best solution if only lowercase letters are used.
    // No hashmap
    fn check_inclusion(s1: String, s2: String) -> bool;

    fn check_inclusion_with_hashmap(s1: String, s2: String) -> bool;
}

impl NewTrait for Solution {
    // Best solution if only lowercase letters are used.
    // No hashmap
    fn check_inclusion(s1: String, s2: String) -> bool {
        if s1.len() > s2.len() {
            return false;
        }

        let mut s1_count = [0; 26];
        let mut window_count = [0; 26];

        for ch in s1.bytes() {
            s1_count[(ch - b'a') as usize] += 1;
        }

        let s1_len = s1.len();
        let s2_bytes: Vec<u8> = s2.bytes().collect();

        for i in 0..s1_len {
            window_count[(s2_bytes[i] - b'a') as usize] += 1;
        }

        if s1_count == window_count {
            return true;
        }

        for i in s1_len..s2_bytes.len() {
            window_count[(s2_bytes[i - s1_len] - b'a') as usize] -= 1;
            window_count[(s2_bytes[i] - b'a') as usize] += 1;

            if s1_count == window_count {
                return true;
            }
        }

        false
    }

    fn check_inclusion_with_hashmap(s1: String, s2: String) -> bool {
        if s1.len() > s2.len() {
            return false;
        }

        let mut s1_freq = HashMap::new();
        let mut window_freq = HashMap::new();

        for ch in s1.chars() {
            *s1_freq.entry(ch).or_insert(0) += 1;
        }

        let s1_len = s1.len();
        let s2_chars: Vec<char> = s2.chars().collect();

        (0..s1_len).for_each(|i| {
            *window_freq.entry(s2_chars[i]).or_insert(0) += 1;
        });

        if s1_freq == window_freq {
            return true;
        }

        for i in s1_len..s2_chars.len() {
            let remove_char = s2_chars[i - s1_len];
            *window_freq.get_mut(&remove_char).unwrap() -= 1;
            if window_freq[&remove_char] == 0 {
                window_freq.remove(&remove_char);
            }

            let add_char = s2_chars[i];
            *window_freq.entry(add_char).or_insert(0) += 1;

            if s1_freq == window_freq {
                return true;
            }
        }

        false
    }
}

#[test]
fn it_works() {
    let s1 = "ab".to_owned();
    let s2 = "eidboaoo".to_owned();

    let output = Solution::check_inclusion(s1, s2);

    println!("{output}");
}
#[test]
fn hashmap_works() {
    let s1 = "ab".to_owned();
    let s2 = "eidboaoo".to_owned();

    let output = Solution::check_inclusion_with_hashmap(s1, s2);

    println!("{output}");
}
