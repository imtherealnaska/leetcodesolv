pub fn can_place_flowers(flowerbed: Vec<i32>, n: i32) -> bool {
    let mut bed = flowerbed;
    let mut count = 0;
    let size = bed.len();

    for i in 0..size {
        if bed[i] == 0 {
            let is_left_empty = i == 0 || bed[i - 1] == 0;
            let is_right_empty = i == size - 1 || bed[i + 1] == 0;

            if is_left_empty && is_right_empty {
                bed[i] = 1;
                count += 1;

                if count >= n {
                    return true;
                }
            }
        }
    }
    count >= n
}
