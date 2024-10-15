use std::usize;

pub fn minimum_steps(s: String) -> i64 {
    let mut steps = 0;
    let mut white_pos = 0;

    for (i, c) in s.chars().enumerate() {
        if c == '0' {
            steps += i as i64 - white_pos as i64;
            white_pos += 1;
        }
    }
    steps
}

pub fn found_nicer_solution(s: String) -> i64 {
    s.chars()
        .enumerate()
        .filter(|(_, ch)| *ch == '0')
        .enumerate()
        .map(|(whites, (i, _))| i - whites)
        .sum::<usize>() as _
}

#[test]
fn nice_steps_it_works() {
    let s = "101".to_owned();
    println!("{}", found_nicer_solution(s));
}

#[test]
fn steps_it_works() {
    let s = "101".to_owned();
    println!("{}", minimum_steps(s));
}
