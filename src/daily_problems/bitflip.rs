pub fn min_bit_flips(start: i32, goal: i32) -> i32 {
    let xor_result = start ^ goal;
    xor_result.count_ones() as i32
}
