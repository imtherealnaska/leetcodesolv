pub fn chalk_replacer(chalk: Vec<i32>, k: i32) -> i32 {
    let s: i64 = chalk.iter().map(|&x| x as i64).sum();
    let mut k = (k as i64) % s;
    println!("k - {k}");
    for (i, &x) in chalk.iter().enumerate() {
        if k < (x as i64) {
            return i as i32;
        }
        k -= x as i64;
        println!("value of k is {k}");
    }
    0
}
