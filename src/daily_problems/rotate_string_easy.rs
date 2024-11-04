pub fn rotate_string(s: String, goal: String) -> bool {
    if s.len() != goal.len() {
        return false;
    }

    if s.is_empty() && goal.is_empty() {
        return true;
    }

    let double = s.clone() + &s;

    double.contains(&goal)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rotate_string() {
        assert!(rotate_string(String::from("abcde"), String::from("cdeab")));
        assert!(rotate_string(String::from("abcde"), String::from("abcde")));
        assert!(!rotate_string(String::from("abcde"), String::from("abced")));
        assert!(rotate_string(String::from(""), String::from("")));
        assert!(!rotate_string(String::from("abc"), String::from("def")));
    }
}
