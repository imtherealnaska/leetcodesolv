pub fn find_min_difference(time_points: Vec<String>) -> i32 {
    // O(n)
    let mut minutes: Vec<i32> = time_points
        .iter()
        .map(|t| {
            let parts: Vec<&str> = t.split(':').collect();
            let hours: i32 = parts[0].parse().unwrap();
            let mins: i32 = parts[1].parse().unwrap();
            hours * 60 + mins
        })
        .collect();

    minutes.sort_unstable();

    let n = minutes.len();
    let mut min_diff = i32::MAX;

    for i in 1..n {
        min_diff = min_diff.min(minutes[i] - minutes[i - 1]);
    }

    min_diff = min_diff.min(1440 + minutes[0] - minutes[n - 1]);
    min_diff
}
