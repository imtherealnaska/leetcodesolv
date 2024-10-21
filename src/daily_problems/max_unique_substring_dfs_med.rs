use std::{char, collections::HashSet};

pub fn max_unique_splits(s: String) -> i32 {
    let chars: Vec<char> = s.chars().collect();
    let n = chars.len();

    fn dfs(index: usize, seen: &mut HashSet<String>, chars: &[char]) -> i32 {
        if index == chars.len() {
            return 0;
        }

        let mut max_splits = 0;
        for i in index..chars.len() {
            let substring: String = chars[index..=i].iter().collect();
            if !seen.contains(&substring) {
                seen.insert(substring.clone());
                let splits = 1 + dfs(i + 1, seen, chars);
                max_splits = max_splits.max(splits);
                seen.remove(&substring);
            }
        }
        max_splits
    }
    dfs(0, &mut HashSet::new(), &chars)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn prob1593_works() {
        let a = "ababccc".to_owned();
        assert_eq!(5, max_unique_splits(a));
    }
}
