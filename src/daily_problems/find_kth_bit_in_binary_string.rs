pub fn find_kth_bit(n: i32, k: i32) -> char {
    if n == 1 {
        return '0';
    }

    let length = (1 << n) - 1;
    let mid = 1 << (n - 1);

    // if k < mid {
    //     find_kth_bit(n - 1, k)
    // } else if k > mid {
    //     if find_kth_bit(n - 1, length - k + 1) == '0' {
    //         '1'
    //     } else {
    //         '0'
    //     }
    // } else {
    //     '1'
    // }
    match k.cmp(&mid) {
        std::cmp::Ordering::Less => find_kth_bit(n - 1, k),
        std::cmp::Ordering::Greater => match find_kth_bit(n - 1, length - k + 1) {
            '0' => '1',
            '1' => '0',
            _ => unreachable!(),
        },
        std::cmp::Ordering::Equal => '1',
    }
}

#[test]
fn find_kth_bit_works() {
    let n = 3;
    let k = 1;

    let res = find_kth_bit(n, k);
    assert_eq!(res, '0');
}
