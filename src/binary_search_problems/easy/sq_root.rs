pub fn my_sqrt(x: i32) -> i32 {
    if x < 2 {
        return x;
    }

    let (mut left, mut right) = (2, x / 2);

    while left <= right {
        let mid = left + ((right - left) >> 1);
        let num = (mid as i64) * (mid as i64);
        match num.cmp(&(x as i64)) {
            std::cmp::Ordering::Less => {
                left = mid + 1;
            }
            std::cmp::Ordering::Equal => {
                return mid;
            }
            std::cmp::Ordering::Greater => {
                right = mid - 1;
            }
        }
    }
    right
}

#[test]
fn sqrt_works() {
    let x = 676;
    assert_eq!(25, my_sqrt(x));
}
