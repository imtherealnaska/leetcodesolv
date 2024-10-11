use std::collections::BTreeSet;

struct Solution;

impl Solution {
    pub fn smallest_chair(times: Vec<Vec<i32>>, target_friend: i32) -> i32 {
        let mut events = Vec::new();
        for (i, time) in times.iter().enumerate() {
            events.push((time[0], 1, i));
            events.push((time[1], 0, i));
        }

        events.sort_unstable();

        let mut available_chairs = BTreeSet::new();
        let mut occupied_chairs = BTreeSet::new();
        let mut friend_chairs = vec![0; times.len()];

        for (_, event_type, friend) in events {
            if event_type == 1 {
                //arrival
                let chair = if let Some(&first_available) = available_chairs.iter().next() {
                    available_chairs.remove(&first_available);
                    first_available
                } else {
                    occupied_chairs.len() as i32
                };

                occupied_chairs.insert(chair);
                friend_chairs[friend] = chair;

                if friend as i32 == target_friend {
                    return chair;
                }
            } else {
                let chair = friend_chairs[friend];
                occupied_chairs.remove(&chair);
                available_chairs.insert(chair);
            }
        }
        unreachable!("Target friend did not  arrive")
    }
}

#[test]
fn it_wrks_chair() {
    let times = vec![vec![1, 4], vec![2, 3], vec![4, 6]];
    let target_friend = 1;
    let result = Solution::smallest_chair(times, target_friend);
    println!("Chair number for target friend: {}", result);
}
