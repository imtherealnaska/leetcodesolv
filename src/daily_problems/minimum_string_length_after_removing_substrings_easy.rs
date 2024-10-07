struct Solution;

impl Solution {
    pub fn min_length(s: String) -> i32 {
        let mut stack: Vec<char> = Vec::new();

        for c in s.chars() {
            if !stack.is_empty()
                && ((c == 'B' && *stack.last().unwrap() == 'A')
                    || (c == 'D' && *stack.last().unwrap() == 'C'))
            {
                stack.pop();
            } else {
                stack.push(c);
            }
        }
        stack.len() as i32
    }
}

#[test]
fn it_works() {
    let s = "ABFCACDB".to_owned();
    assert_eq!(Solution::min_length(s), 2);
}
