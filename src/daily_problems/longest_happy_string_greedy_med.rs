use std::cmp::Reverse;
use std::collections::BinaryHeap;

struct CharCount {
    ch: char,
    count: i32,
}

impl CharCount {
    fn new(ch: char, count: i32) -> Self {
        Self { ch, count }
    }
}

fn longest_diverse_string(a: i32, b: i32, c: i32) -> String {
    let mut heap = BinaryHeap::new();
    if a > 0 {
        heap.push((a, Reverse('a')));
    }
    if b > 0 {
        heap.push((b, Reverse('b')));
    }
    if c > 0 {
        heap.push((c, Reverse('c')));
    }

    let mut result = String::new();
    let mut prev = CharCount::new(' ', 0);
    let mut prev_prev = CharCount::new(' ', 0);

    while let Some((count, Reverse(ch))) = heap.pop() {
        if (prev.ch == ch && prev_prev.ch == ch) || count == 0 {
            if let Some((next_count, Reverse(next_ch))) = heap.pop() {
                result.push(next_ch);
                if next_count > 1 {
                    heap.push((next_count - 1, Reverse(next_ch)));
                }
                heap.push((count, Reverse(ch)));
                prev_prev = prev;
                prev = CharCount::new(next_ch, 1);
            } else {
                break;
            }
        } else {
            result.push(ch);
            if count > 1 {
                heap.push((count - 1, Reverse(ch)));
            }
            prev_prev = prev;
            prev = CharCount::new(ch, 1);
        }
    }

    result
}
