pub fn find_the_longest_substring(s: String) -> i32 {
    let mut state = 0u32;
    let mut seen = [usize::MAX; 32];
    seen[0] = 0;
    let mut max_len = 0;

    for (i, &b) in s.as_bytes().iter().enumerate() {
        state ^= match b {
            b'a' => 1,
            b'e' => 2,
            b'i' => 4,
            b'o' => 8,
            b'u' => 16,
            _ => 0,
        };

        let prev = seen[state as usize];
        if prev == usize::MAX {
            seen[state as usize] = i + 1;
        } else {
            max_len = max_len.max(i + 1 - prev);
        }
    }
    max_len as i32
}
