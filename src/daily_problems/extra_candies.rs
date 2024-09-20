pub fn kids_with_candies(candies: Vec<i32>, extra_candies: i32) -> Vec<bool> {
    let max_candies = *candies.iter().max().unwrap_or(&0);
    candies
        .iter()
        .map(|candy| candy + extra_candies >= max_candies)
        .collect()
}
