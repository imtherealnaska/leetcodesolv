pub mod daily_problems;
pub mod sorts;
mod binary_search_problems;

use daily_problems::*;

fn main() {
    //    let nums = vec![1, 2, 3, 4, 5, 6, 7];
    //
    //    let target = 5;
    //    let ans = two_sum(nums, target);
    //    println!("{ans:?}");
    //    let snums = vec![3, 4, 1, 2];
    //    let k = 25;
    //    let sans = chalk_replacer(snums, k);
    //    println!("{sans:?}");
    //
    //    let a = get_lucky("iiii".to_string(), 1);
    //    println!("a = {a}");
    //
    //    let word1 = "abc".to_string();
    //    let word2 = "pqr".to_string();
    //    println!("{}", merge_alternatively(word1, word2));
    //let str1 = "ABCABC".to_string();
    //let str2 = "ABC".to_string();
    //let gcd_string = gcd_of_strings(str1, str2);
    //println!("{}", gcd_string);

    //let commands = vec![6, -1, -1, 6];
    //let obstacles: Vec<Vec<i32>> = Vec::new();
    //let result = walking_robot::robot_sim(commands, obstacles);
    //println!("{result}");

    // let candies = vec![2,3,5,1,3];
    // let extra_canies = 3 ;
    // println!("{:?}" , extra_candies::kids_with_candies(candies, extra_canies));
    // let rolls = vec![3, 2, 4, 3];
    // let mean = 4;
    // let n = 2;
    // let result = missing_rolls(rolls, mean, n);
    // println!("{:?}", result);
    // println!("{}", reverse_vowels("hello".to_string())); // Output: "holle"
    // println!("{}", reverse_vowels("leetcode".to_string())); // Output: "leotcede"

    let start = 3;
    let goal = 4;
    println!("{}", bitflip::min_bit_flips(start, goal));

    let s = String::from("leetcodeisgreat");
    println!(
        "{}",
        longest_substring_containing_vowels_even_med::find_the_longest_substring(s)
    );
}
