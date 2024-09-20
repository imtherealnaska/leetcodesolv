use std::collections::HashMap;

pub fn uncommon_from_sentences(s1: String, s2: String) -> Vec<String> {
    let mut word_count = HashMap::new();

    for word in s1.split_whitespace().chain(s2.split_whitespace()) {
        *word_count.entry(word.to_string()).or_insert(0) += 1;
    }

    word_count
        .into_iter()
        .filter(|(_, count)| *count == 1)
        .map(|(word, _)| word)
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_uncommon_words() {
        let s1 = String::from("Hi my name is narendra");
        let s2 = String::from("Hi my name is deepak");
        let result = uncommon_from_sentences(s1, s2);
        assert_eq!(result, vec!["deepak" , "narendra"]);
    }
}
