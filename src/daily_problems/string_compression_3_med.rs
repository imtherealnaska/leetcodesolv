pub fn compressed_string(word: String) -> String {
    let mut comp = String::new();
    let mut chars = word.chars().peekable();

    while let Some(&current_char) = chars.peek() {
        let mut count = 0;
        while count < 9 && chars.peek() == Some(&current_char) {
            count += 1;
            chars.next();
        }
        comp.push_str(&count.to_string());
        comp.push(current_char);
    }
    comp
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_compression() {
        assert_eq!(compressed_string("a".to_string()), "1a".to_string());
        assert_eq!(compressed_string("aaa".to_string()), "3a".to_string());
        assert_eq!(
            compressed_string("aaaaaaaaaa".to_string()),
            "9a1a".to_string()
        ); // 10 'a.to_string()'s
        assert_eq!(compressed_string("aabbb".to_string()), "2a3b".to_string());
        assert_eq!(
            compressed_string("aaabbbcccc".to_string()),
            "3a3b4c".to_string()
        );
        assert_eq!(compressed_string("".to_string()), "".to_string());
    }
}
