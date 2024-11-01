pub fn minimum_total_distance(mut robot: Vec<i32>, mut factory: Vec<Vec<i32>>) -> i64 {
    robot.sort_unstable();
    factory.sort_unstable_by_key(|f| f[0]);

    let n = robot.len();
    let m = factory.len();

    let mut dp = vec![vec![None; m + 1]; n + 1];

    fn solve(
        pos: usize,
        fact_pos: usize,
        robot: &[i32],
        factory: &[Vec<i32>],
        dp: &mut Vec<Vec<Option<i64>>>,
    ) -> i64 {
        if pos >= robot.len() {
            return 0;
        }
        if fact_pos >= factory.len() {
            return i64::MAX / 2;
        }

        if let Some(result) = dp[pos][fact_pos] {
            return result;
        }

        let mut min_dist = solve(pos, fact_pos + 1, robot, factory, dp);

        let mut curr_dist = 0;
        let limit = factory[fact_pos][1] as usize;
        let factory_pos = factory[fact_pos][0];

        for k in 0..limit.min(robot.len() - pos) {
            curr_dist += (robot[pos + k] - factory_pos).abs() as i64;
            let next_dist = solve(pos + k + 1, fact_pos + 1, robot, factory, dp);

            if next_dist != i64::MAX / 2 {
                min_dist = min_dist.min(curr_dist + next_dist);
            }
        }
        dp[pos][fact_pos] = Some(min_dist);
        min_dist
    }
    solve(0, 0, &robot, &factory, &mut dp)
}
