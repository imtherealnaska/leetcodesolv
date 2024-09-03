pub(crate) fn get_lucky(s: String, k: i32) -> i32 {
    let mut num_string: String = s
        .chars()
        .map(|c| ((c as u8 - b'a' + 1) as u32).to_string())
        .collect();

    for _ in 0..k {
        num_string = num_string
            .chars()
            .map(|d| d.to_digit(10).unwrap())
            .sum::<u32>()
            .to_string();
    }
    num_string.parse().unwrap()
}
