struct Solution;

impl Solution {
    pub fn lexical_order(n: i32) -> Vec<i32> {
        let mut result = Vec::with_capacity(n as usize);
        let mut current = 1;

        for _ in 0..n {
            result.push(current);

            if current * 10 <= n {
                current *= 10;
            } else {
                if current >= n {
                    current /= 10;
                }
                current += 1;

                while current % 10 == 0 {
                    current /= 10;
                }
            }
        }
        result
    }
}
#[test]
fn lex_order_passes() {
    let n = 12;
    println!("{:?}", Solution::lexical_order(n));
}
