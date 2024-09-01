use twosum::{no_hashmap_two_sum, two_sum};

pub mod twosum;

fn main() {
    let nums = vec![1, 2, 3, 4, 5, 6, 7];

    let target = 5;
    let ans = two_sum(nums, target);
    println!("{ans:?}");
    let snums = vec![1, 2, 3, 4, 5, 6, 7];

    let sans = no_hashmap_two_sum(snums, target);
    println!("{sans:?}");
}
