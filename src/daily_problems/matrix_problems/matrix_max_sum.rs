pub fn max_matrix_sum(matrix: Vec<Vec<i32>>) -> i64 {
    let mut count_negative = 0;
    let mut total_sum: i64 = 0;
    let mut min_abs = i32::MAX;

    for row in matrix.iter() {
        for &num in row.iter() {
            if num < 0 {
                count_negative += 1;
            }

            min_abs = min_abs.min(num.abs());

            total_sum += num.abs() as i64;
        }
    }

    if count_negative % 2 == 0 {
        total_sum
    } else {
        total_sum - 2 * (min_abs as i64)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic_matrix() {
        let matrix = vec![vec![1, 2, 3], vec![-1, -2, -3], vec![1, 2, 3]];
        assert_eq!(max_matrix_sum(matrix), 16);
    }

    #[test]
    fn test_all_positive() {
        let matrix = vec![vec![1, 2], vec![3, 4]];
        assert_eq!(max_matrix_sum(matrix), 10);
    }

    #[test]
    fn test_all_negative() {
        let matrix = vec![vec![-1, -2], vec![-3, -4]];
        assert_eq!(max_matrix_sum(matrix), 10);
    }

    #[test]
    fn test_mixed_values() {
        let matrix = vec![vec![-1, 1], vec![1, -1]];
        assert_eq!(max_matrix_sum(matrix), 4);
    }

    #[test]
    fn test_single_element() {
        let matrix = vec![vec![-5]];
        assert_eq!(max_matrix_sum(matrix), -5);
    }

    #[test]
    fn test_zero_included() {
        let matrix = vec![vec![0, -1], vec![-2, 3]];
        assert_eq!(max_matrix_sum(matrix), 6);
    }
}
