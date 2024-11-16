pub fn minimized_maximum(n: i32, quantities: Vec<i32>) -> i32 {
    let mut left = 1;
    let mut right = *quantities.iter().max().unwrap();

    while left < right {
        let mid = (left + right) / 2;

        if can_distribute(mid, n, &quantities) {
            right = mid;
        } else {
            left = mid + 1;
        }
    }

    left
}

fn can_distribute(max_items: i32, stores: i32, quantities: &Vec<i32>) -> bool {
    let mut required_stores = 0;

    for &quantity in quantities {
        required_stores += (quantity + max_items - 1) / max_items; // ceil(quantity / max_items)
    }

    required_stores <= stores
}

fn another_tricky(n: i32, quantities: Vec<i32>) -> i32 {
    let ratio_array = calculate_ratio_array(quantities);
    0
}

fn calculate_ratio_array(quantities: Vec<i32>) -> Vec<i32> {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn minimise_max_tricky_works() {
        let quantities = vec![15, 10, 10];
        let n = 7;
        assert_eq!(5, another_tricky(n, quantities));
    }

    #[test]
    fn minimise_max_normal_works() {
        let quantities = vec![15, 10, 10];
        let n = 7;
        assert_eq!(5, minimized_maximum(n, quantities));
    }
}
