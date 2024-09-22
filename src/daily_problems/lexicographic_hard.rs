pub struct Solution;

impl Solution {
    pub fn lexical_order(n: i32, k: i32) -> i32 {
        let mut current = 1;
        let mut k = k - 1;

        while k > 0 {
            let steps = Self::count_steps(n, current, current + 1);

            if steps <= k {
                current += 1;
                k -= steps;
            } else {
                current *= 10;
                k -= 1;
            }
        }
        current as i32
    }

    fn count_steps(n: i32, n1: i64, n2: i64) -> i32 {
        let mut steps = 0;
        let mut first = n1;
        let mut last = n2;

        while first <= n as i64 {
            steps += (last.min(n as i64 + 1) - first) as i32;
            first *= 10;
            last *= 10
        }
        steps
    }
}

#[test]
fn hard_lex_order_passes() {
    let n = 804289384;
    let k = 42641503;

    println!("{:?}", Solution::lexical_order(n, k));
}
