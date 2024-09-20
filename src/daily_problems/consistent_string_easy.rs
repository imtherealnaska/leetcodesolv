use std::collections::HashSet;

pub fn count_consistent_strings(allowed: String, words: Vec<String>) -> i32 {
    let allowed_set: HashSet<char> = allowed.chars().collect();

    words
        .into_iter()
        .filter(|word| word.chars().all(|c| allowed_set.contains(&c)))
        .count() as i32
}
