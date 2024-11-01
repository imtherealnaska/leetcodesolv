pub fn make_fancy_string(s: String) -> String {
    let mut result = Vec::new();

    for c in s.chars() {
        if result.len() >= 2 && result[result.len() - 1] == c && result[result.len() - 2] == c {
            continue;
        }
        result.push(c);
    }

    result.into_iter().collect()
}
