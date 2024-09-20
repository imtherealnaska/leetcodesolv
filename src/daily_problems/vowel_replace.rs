use std::ascii::AsciiExt;

pub fn reverse_vowels(s: String) -> String {
    let mut chars: Vec<char> = s.chars().collect();
    let mut left = 0;
    let mut right = chars.len() - 1;

    while left < right {
        if !is_vowel(chars[left]) {
            left += 1;
        } else if !is_vowel(chars[right]) {
            right -= 1;
        } else {
            chars.swap(left, right);
            left += 1;
            right -= 1;
        }
    }
    chars.into_iter().collect()
}

fn is_vowel(c: char) -> bool {
    matches!(c.to_ascii_lowercase(), 'a' | 'e' | 'i' | 'o' | 'u')
}
