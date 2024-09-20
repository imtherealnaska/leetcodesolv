pub fn largest_number(nums: Vec<i32>) -> String {
    let mut num_str: Vec<String> = nums.into_iter().map(|num| num.to_string()).collect();

    num_str.sort_unstable_by(|a, b| {
        let order1 = a.clone() + b;
        let order2 = b.clone() + a;
        order2.cmp(&order1)
    });

    let result = num_str.concat();

    if result.starts_with('0') {
        "0".to_string()
    } else {
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_larget_number() {
        let nums = vec![10, 2];
        let ans = largest_number(nums);
        assert_eq!(ans, "210");
    }
}
