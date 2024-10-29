pub fn max_moves(grid: Vec<Vec<i32>>) -> i32 {
    let m = grid.len();
    let n = grid[0].len();
    //construct a matrix
    let mut memo = vec![vec![-1; n]; m];

    fn dfs(grid: &Vec<Vec<i32>>, memo: &mut Vec<Vec<i32>>, row: usize, col: usize) -> i32 {
        if col == grid[0].len() - 1 {
            return 0;
        }

        if memo[row][col] != -1 {
            return memo[row][col];
        }

        let directions = [(-1, 1), (0, 1), (1, 1)];
        let mut max_moves = 0;

        for (dr, dc) in directions.iter() {
            let new_row = row as i32 + dr;
            let new_col = col as i32 + dc;
            if new_row >= 0 && new_row < grid.len() as i32 && new_col < grid[0].len() as i32 {
                let new_row = new_row as usize;
                let new_col = new_col as usize;

                if grid[new_row][new_col] > grid[row][col] {
                    max_moves = max_moves.max(1 + dfs(grid, memo, new_row, new_col));
                }
            }
        }
        memo[row][col] = max_moves;
        max_moves
    }
    let mut result = 0;
    for row in 0..m {
        result = result.max(dfs(&grid, &mut memo, row, 0));
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn grid_max_moves() {
        let grid = vec![vec![3, 2, 4], vec![2, 1, 9], vec![1, 1, 7]];
        println!("Maximum moves: {}", max_moves(grid));
    }
}
