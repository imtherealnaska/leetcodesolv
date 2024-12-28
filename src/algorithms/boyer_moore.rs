use std::collections::HashMap;

pub struct BoyerMoore {
    bad_char: HashMap<char, usize>,
    goood_suffix: Vec<usize>,
    pattern: String,
}

impl BoyerMoore {
    pub fn new(pattern: &str) -> Self {
        let bad_char = Self::build_bad_char_table(pattern);
        let goood_suffix = Self::build_good_suffix_table(pattern);

        BoyerMoore {
            bad_char,
            goood_suffix,
            pattern: pattern.to_string(),
        }
    }

    fn build_good_suffix_table(pattern: &str) -> Vec<usize> {
        let n = pattern.len();
        let mut good_suffix = vec![n; n];
        let mut last_prefix_pos = n;
        for i in (0..n - 1).rev() {
            if Self::is_prefix(pattern, i + 1) {
                last_prefix_pos = i + 1;
            }
            good_suffix[i] = last_prefix_pos + n - 1 - i;
        }

        for i in 0..n - 1 {
            let suffix_len = Self::get_suffix_length(pattern, i);
            if suffix_len > 0 {
                good_suffix[n - 1 - suffix_len] = n - 1 - i + n - suffix_len;
            }
        }
        good_suffix
    }

    fn build_bad_char_table(pattern: &str) -> HashMap<char, usize> {
        let mut bad_char = HashMap::new();
        for (i, c) in pattern.chars().enumerate() {
            bad_char.insert(c, i);
        }
        bad_char
    }

    fn get_suffix_length(pattern: &str, pos: usize) -> usize {
        let chars: Vec<char> = pattern.chars().collect();
        let n = chars.len();
        let mut length = 0;
        let mut i = pos as i32;
        let mut j = (n - 1) as i32;

        while i >= 0 && chars[i as usize] == chars[j as usize] {
            length += 1;
            i -= 1;
            j -= 1;
        }
        length
    }

    fn is_prefix(pattern: &str, pos: usize) -> bool {
        let chars: Vec<char> = pattern.chars().collect();
        let n = chars.len();
        let mut j = 0;

        for i in pos..n {
            if chars[i] != chars[j] {
                return false;
            }
            j += 1;
        }
        true
    }

    pub fn search(&self, text: &str) -> Vec<usize> {
        let mut matches = Vec::new();
        if self.pattern.is_empty() || text.is_empty() {
            return matches;
        }

        let text_chars: Vec<char> = text.chars().collect();
        let patterN_chars: Vec<char> = self.pattern.chars().collect();
        let n = text.len();
        let m = self.pattern.len();
        let mut i = 0;

        while i <= n - m {
            let mut j = m as i32 - 1;
            while j >= 0 && patterN_chars[j as usize] == text_chars[i + j as usize] {
                j -= 1;
            }

            if j < 0 {
                matches.push(i);
                i += if m > 1 { self.goood_suffix[0] } else { 1 }
            } else {
                let bc_shift = j as usize
                    - self
                        .bad_char
                        .get(&text_chars[i + j as usize])
                        .copied()
                        .unwrap_or(usize::MAX);
                let gs_shift = self.goood_suffix[j as usize];
                i += bc_shift.max(gs_shift);
            }
        }
        matches
    }
}
