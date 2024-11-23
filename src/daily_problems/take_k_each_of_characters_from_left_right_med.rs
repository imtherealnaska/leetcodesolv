pub fn take_characters(s: String, k: i32) -> i32 {
    use std::collections::HashMap;

    let mut char_count = HashMap::new();
    for ch in s.chars() {
        *char_count.entry(ch).or_insert(0) += 1;
    }

    if *char_count.get(&'a').unwrap_or(&0) < k
        || *char_count.get(&'b').unwrap_or(&0) < k
        || *char_count.get(&'c').unwrap_or(&0) < k
    {
        return -1;
    }

    let mut left = 0;
    let mut right = s.len();
    let mut counts = HashMap::new();

    counts.insert('a', 0);
    counts.insert('b', 0);
    counts.insert('c', 0);

    let mut min_length = usize::MAX;
    while left < right {
        if counts[&'a'] >= k && counts[&'b'] >= k && counts[&'c'] >= k {
            min_length = min_length.min(right - left);
            if let Some(count) = counts.get_mut(&s[left..left + 1].chars().next().unwrap()) {
                *count -= 1;
            }
            left += 1;
        } else {
            right -= 1;
            if let Some(count) = counts.get_mut(&s[right..right + 1].chars().next().unwrap()) {
                *count += 1;
            }
        }
    }
    min_length as i32
}

pub fn take_chars_2(s: String, k: i32) -> i32 {
    let n = s.len();
    let k = k as usize;

    let s: Vec<char> = s.chars().collect();

    let mut total = vec![0; 3];

    for &c in &s {
        total[(c as usize) - ('a' as usize)] += 1;
    }

    if total.iter().any(|&x| x < k) {
        return -1;
    }

    let mut curr = vec![0; 3];
    let mut left = 0;
    let mut max_valid_window = 0;

    for right in 0..n {
        curr[(s[right] as usize) - ('a' as usize)] += 1;

        while left <= right
            && (total[0] - curr[0] < k || total[1] - curr[1] < k || total[2] - curr[2] < k)
        {
            curr[(s[left] as usize) - ('a' as usize)] -= 1;
            left += 1;
        }
        max_valid_window = max_valid_window.max(right - left + 1);
    }
    (n - max_valid_window) as i32
}
