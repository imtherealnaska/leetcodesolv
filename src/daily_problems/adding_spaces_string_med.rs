pub fn add_spaces(s: String, spaces: Vec<i32>) -> String {
    let mut result = String::with_capacity(s.len() + spaces.len());
    let mut chars = s.chars();
    let mut last_idx = 0;

    for &space_idx in spaces.iter() {
        while last_idx < space_idx {
            result.push(chars.next().unwrap());
            last_idx += 1;
        }
        result.push(' ');
    }
    result.extend(chars);
    result
}
