use std::collections::HashMap;

fn max_length(s: String) -> i32 {
    let mut count: HashMap<(char, usize), usize> = HashMap::new();

    for start in 0..s.len() {
        let chars: Vec<char> = s.chars().collect();
        let character = chars[start];
        let mut substring_length = 0;

        for end in start..s.len() {
            if character == chars[end] {
                substring_length += 1;
                *count.entry((character, substring_length)).or_insert(0) += 1;
            } else {
                break;
            }
        }
    }
    let ans = count
        .iter()
        .filter(|&(_, &freq)| freq >= 3)
        .map(|(&(_, length), _)| length)
        .max()
        .unwrap_or(0);

    if ans == 0 {
        -1
    } else {
        ans as i32
    }
}
