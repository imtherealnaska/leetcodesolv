#[inline]
fn count_bits(n: i32) -> i32 {
    n.count_ones() as i32
}

fn other_count_bits(n: i32) -> i32 {
    let mut num = n;
    let mut count = 0;

    while num > 0 {
        count += num & 1;
        num >>= 1;
    }
    count
}

fn can_sort_array(nums: Vec<i32>) -> bool {
    if nums.len() <= 1 {
        return true;
    }

    let mut pairs: Vec<(i32, i32)> = nums.iter().map(|&x| (x, count_bits(x))).collect();

    let mut i = 0;
    while i < pairs.len() {
        let mut j = i + 1;
        while j < pairs.len() && pairs[j].1 == pairs[i].1 {
            j += 1;
        }

        if j - i > 1 {
            pairs[i..j].sort_by_key(|&x| x.0);
        }

        i = j;
    }

    for i in 1..pairs.len() {
        if pairs[i].0 < pairs[i - 1].0 {
            return false;
        }
    }
    true
}
