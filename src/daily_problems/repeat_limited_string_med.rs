use std::collections::{BinaryHeap, HashMap};

pub fn repeat_limited_string_med(s: String, repeat_limit: i32) -> String {
    let mut freq = HashMap::new();
    for ch in s.chars() {
        *freq.entry(ch).or_insert(0) += 1;
    }

    let mut max_heap: BinaryHeap<char> = freq.keys().cloned().collect();

    let mut result = String::new();

    while !max_heap.is_empty() {
        let ch = max_heap.pop().unwrap();
        let count = *freq.get(&ch).unwrap();

        let use_count = count.min(repeat_limit);
        result.push_str(&ch.to_string().repeat(use_count as usize));
        freq.insert(ch, count - use_count);

        if freq.get(&ch).unwrap_or(&0) > &0 && !max_heap.is_empty() {
            let next_ch = max_heap.pop().unwrap();
            result.push(next_ch);

            let next_count = *freq.get(&next_ch).unwrap();
            freq.insert(next_ch, next_count - 1);

            if next_count > 1 {
                max_heap.push(next_ch);
            }
            max_heap.push(ch);
        }
    }
    result
}
