struct Solution;

impl Solution {
    pub fn divide_players(mut skill: Vec<i32>) -> i64 {
        let n = skill.len();
        if n % 2 != 0 {
            return -1;
        }

        skill.sort_unstable();

        let team_skill = skill[0] + skill[n - 1];
        let mut chemistry_sum: i64 = 0;

        for i in 0..n / 2 {
            if skill[i] + skill[n - 1 - i] != team_skill {
                return -1;
            }
            chemistry_sum += (skill[i] as i64) * (skill[n - 1 - i] as i64);
        }
        chemistry_sum
    }
}
