use std::collections::HashMap;

pub fn diff_ways_to_compute(expression: String) -> Vec<i32> {
    fn compute(expression: &str, memo: &mut HashMap<String, Vec<i32>>) -> Vec<i32> {
        if let Some(result) = memo.get(expression) {
            return result.clone();
        }
        let mut results = Vec::new();

        for (i, c) in expression.chars().enumerate() {
            if c == '+' || c == '-' || c == '*' {
                let left_part = &expression[0..i];
                let right_part = &expression[i + 1..];

                let left_results = compute(left_part, memo);
                let right_results = compute(right_part, memo);

                for &left in &left_results {
                    for &right in &right_results {
                        let result = match c {
                            '+' => left + right,
                            '-' => left - right,
                            '*' => left * right,
                            _ => unreachable!(),
                        };
                        results.push(result);
                    }
                }
            }
        }
        if results.is_empty() {
            results.push(expression.parse().unwrap());
        }
        memo.insert(expression.to_string(), results.clone());
        results
    }

    let mut memo = HashMap::new();
    compute(&expression, &mut memo)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_evaluation() {
        let ans = diff_ways_to_compute("2*3-4*5".to_string());
        println!("{ans:?}");
    }
}
