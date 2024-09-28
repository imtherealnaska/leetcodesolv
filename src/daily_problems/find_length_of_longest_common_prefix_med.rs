use std::collections::HashSet;

struct Solution;

fn len_of(mut i: i32) -> i32 {
    let mut len = 0;
    while i > 0 {
        i /= 10;
        len += 1;
    }

    len
}

impl Solution {
    pub fn longest_common_prefix(mut arr1: Vec<i32>, mut arr2: Vec<i32>) -> i32 {
        if arr1.len() > arr2.len() {
            std::mem::swap(&mut arr1, &mut arr2);
        }

        let mut prefixes = HashSet::new();
        for mut i in arr1 {
            while i > 0 {
                prefixes.insert(i);
                i /= 10;
            }
        }

        let mut longest = 0;
        for mut i in arr2 {
            while i > 0 {
                if let Some(p) = prefixes.get(&i) {
                    longest = longest.max(len_of(i));
                    break;
                }

                i /= 10;
            }
        }

        longest
    }
}
