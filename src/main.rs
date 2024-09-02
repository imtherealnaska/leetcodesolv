use chalk_med::chalk_replacer;
use twosum::two_sum;

pub mod chalk_med;
pub mod twosum;

fn main() {
    let nums = vec![1, 2, 3, 4, 5, 6, 7];

    let target = 5;
    let ans = two_sum(nums, target);
    println!("{ans:?}");
    let snums = vec![3, 4, 1, 2];
    let k = 25;
    let sans = chalk_replacer(snums, k);
    println!("{sans:?}");
}
