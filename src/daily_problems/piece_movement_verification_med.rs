pub fn can_change(start: String, target: String) -> bool {
    let start_chars: Vec<char> = start.chars().collect();
    let target_chars: Vec<char> = target.chars().collect();

    let start_length = start_chars.len();

    let mut start_index = 0;
    let mut target_index = 0;

    while start_index < start_length || target_index < start_length {
        while start_index < start_length && start_chars[start_index] == '_' {
            start_index += 1;
        }

        while target_index < start_length && target_chars[target_index] == '_' {
            target_index += 1;
        }

        if start_index == start_length || target_index == start_length {
            return start_index == start_length && target_index == start_length;
        }

        if start_chars[start_index] != target_chars[target_index]
            || (start_chars[start_index] == 'L' && start_index < target_index)
            || (start_chars[start_index] == 'R' && start_index > target_index)
        {
            return false;
        }

        start_index += 1;
        target_index += 1;
    }
    true
}
