pub fn min_groups(intervals: Vec<Vec<i32>>) -> i32 {
    let mut events = Vec::with_capacity(intervals.len() * 2);

    for interval in intervals {
        events.push((interval[0], 1));
        events.push((interval[1] + 1, -1));
    }

    events.sort_unstable();

    let mut active_intervals = 0;
    let mut max_active = 0;

    for (_, event_type) in events {
        active_intervals += event_type;
        max_active = max_active.max(active_intervals);
    }
    max_active
}

#[test]
fn it_works() {
    let intervals = vec![vec![5, 10], vec![6, 8], vec![1, 5], vec![2, 3], vec![1, 10]];
    println!("Minimum number of groups: {}", min_groups(intervals));
}
