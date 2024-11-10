pub fn min_end(n: i32, x: i32) -> i64 {
    let mut result = x as i64;
    let mut n = n - 1;
    let mut mask = 1;

    while n > 0 {
        if (mask & (x as i64)) == 0 {
            result |= (n & 1) as i64 * mask;
            n >>= 1;
        }
        mask <<= 1;
    }
    result
}
