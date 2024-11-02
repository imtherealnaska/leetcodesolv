pub fn is_circular_sentence(sentence: String) -> bool {
    if sentence.trim().is_empty() {
        return false;
    }

    let words: Vec<&str> = sentence.split_whitespace().collect();

    // just  need to handle edge case when there is only one word.
    // it should start and end with the same letter
    if words.len() == 1 {
        let word = words[0];
        if !word.chars().all(|c| c.is_ascii_alphabetic()) {
            return false;
        }

        let first = word.chars().next().unwrap();
        let last = word.chars().last().unwrap();
        return first == last;
    }

    if !words
        .iter()
        .all(|word| word.chars().all(|c| c.is_ascii_alphabetic()))
    {
        return false;
    }

    for i in 0..words.len() {
        let current_word = words[i];
        let next_word = words[(i + 1) % words.len()];

        let last_char = current_word.chars().last().unwrap();
        let first_char = next_word.chars().next().unwrap();

        if last_char != first_char {
            return false;
        }
    }
    true
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_circular_sentence() {
        assert!(is_circular_sentence(
            "leetcode exercises sound delightful".to_string()
        ),);
        assert!(is_circular_sentence("eetcode".to_string()));
        assert!(is_circular_sentence("leetcode eats soul".to_string()));
        assert!(!is_circular_sentence("Leetcode is cool".to_string()));
        assert!(!is_circular_sentence("happy Leetcode".to_string()));
        assert!(!is_circular_sentence("Leetcode".to_string()));
        assert!(!is_circular_sentence("I like Leetcode".to_string()));
        assert!(!is_circular_sentence("".to_string()));
        assert!(!is_circular_sentence("  ".to_string()));
    }
}
