pub fn rotate_the_box(rbox: Vec<Vec<char>>) -> Vec<Vec<char>> {
    let m = rbox.len();
    let n = rbox[0].len();

    let mut rotated = vec![vec!['.'; m]; n];

    for i in 0..m {
        for j in 0..n {
            rotated[j][m - 1 - i] = rbox[i][j];
        }
    }

    for col in 0..m {
        let mut bottom = n - 1;
        for row in (0..n).rev() {
            match rotated[row][col] {
                '#' => {
                    if row != bottom {
                        rotated[bottom][col] = '#';
                        rotated[row][col] = '.';
                    }
                    bottom -= 1;
                }
                '*' => {
                    bottom = row - 1;
                }
                '.' => {}
                _ => panic!("Invalid character in box"),
            }
        }
    }
    rotated
}

// Tests
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic_case() {
        let input = vec![
            vec!['.', '#', '.'],
            vec!['.', '#', '.'],
            vec!['#', '*', '.'],
        ];
        let expected = vec![
            vec!['#', '.', '.'],
            vec!['*', '#', '#'],
            vec!['.', '.', '.'],
        ];
        assert_eq!(rotate_the_box(input), expected);
    }

    #[test]
    fn test_complex_case() {
        let input = vec![
            vec!['#', '.', '*', '.'],
            vec!['#', '#', '*', '.'],
            vec!['#', '.', '.', '.'],
        ];
        let expected = vec![
            vec!['.', '#', '#', '#'],
            vec!['.', '.', '.', '#'],
            vec!['*', '*', '.', '.'],
        ];
        assert_eq!(rotate_the_box(input), expected);
    }

    #[test]
    fn test_empty_box() {
        let input = vec![
            vec!['.', '.', '.'],
            vec!['.', '.', '.'],
            vec!['.', '.', '.'],
        ];
        let expected = vec![
            vec!['.', '.', '.'],
            vec!['.', '.', '.'],
            vec!['.', '.', '.'],
        ];
        assert_eq!(rotate_the_box(input), expected);
    }

    #[test]
    fn test_all_stones() {
        let input = vec![vec!['#', '#', '#'], vec!['#', '#', '#']];
        let expected = vec![vec!['#', '#'], vec!['#', '#'], vec!['#', '#']];
        assert_eq!(rotate_the_box(input), expected);
    }
}
