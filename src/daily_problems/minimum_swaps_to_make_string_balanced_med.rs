fn min_swaps(s: String) -> i32 {
    let mut imbalance = 0;
    let mut max_imbalance = 0;

    for ch in s.chars() {
        if ch == '[' {
            imbalance -= 1;
        } else {
            imbalance += 1;
        }
        max_imbalance = max_imbalance.max(imbalance);
    }
    (max_imbalance + 1) / 2
}
