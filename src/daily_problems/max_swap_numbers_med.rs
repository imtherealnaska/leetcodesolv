pub fn maximum_swap(num: i32) -> i32 {
    let mut digits: Vec<i32> = num
        .to_string()
        .chars()
        .map(|c| c.to_digit(10).unwrap() as i32)
        .collect();

    let mut last: [usize; 10] = [0; 10];

    for (i, &digit) in digits.iter().enumerate() {
        last[digit as usize] = i;
    }

    for i in 0..digits.len() {
        for d in (digits[i] + 1..10).rev() {
            if last[d as usize] > i {
                digits.swap(i, last[d as usize]);
                return digits.iter().fold(0, |acc, &x| acc * 10 + x);
            }
        }
    }
    num
}
