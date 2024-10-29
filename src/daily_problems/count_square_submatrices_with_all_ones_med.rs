pub fn count_squares(matrix: Vec<Vec<i32>>) -> i32 {
    if matrix.is_empty() || matrix[0].is_empty() {
        return 0;
    }

    let m = matrix.len();
    let n = matrix[0].len();

    let mut dp = vec![vec![0; n]; m];
    let mut result = 0;

    for j in 0..n {
        dp[0][j] = matrix[0][j];
        result += dp[0][j];
    }

    for i in 1..m {
        dp[i][0] = matrix[i][0];
        result += dp[i][0];
    }

    for i in 1..m {
        for j in 1..n {
            if matrix[i][j] == 1 {
                dp[i][j] =
                    std::cmp::min(dp[i - 1][j - 1], std::cmp::min(dp[i - 1][j], dp[i][j - 1])) + 1;
                result += dp[i][j];
            }
        }
    }
    result
}

pub fn count_squares_editorial(matrix: Vec<Vec<i32>>) -> i32 {
    let (row, col) = (matrix.len(), matrix[0].len());
    let mut dp = vec![vec![0; col + 1]; row + 1];
    let mut ans = 0;

    for i in 0..row {
        for j in 0..col {
            if matrix[i][j] == 1 {
                dp[i + 1][j + 1] =
                    std::cmp::min(dp[i][j + 1], std::cmp::min(dp[i + 1][j], dp[i][j])) + 1;
                ans += dp[i + 1][j + 1];
            }
        }
    }
    ans
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn squares_works() {
        let matrix = vec![vec![0, 1, 1, 1], vec![1, 1, 1, 1], vec![0, 1, 1, 1]];
        assert_eq!(count_squares(matrix), 15);
        let matrix = vec![vec![1, 0, 1], vec![1, 1, 0], vec![1, 1, 0]];
        assert_eq!(count_squares(matrix), 7);
    }

    #[test]
    fn edit_squares_works() {
        let matrix = vec![vec![0, 1, 1, 1], vec![1, 1, 1, 1], vec![0, 1, 1, 1]];
        assert_eq!(count_squares_editorial(matrix), 15);
        let matrix = vec![vec![1, 0, 1], vec![1, 1, 0], vec![1, 1, 0]];
        assert_eq!(count_squares_editorial(matrix), 7);
    }
}
