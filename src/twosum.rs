use std::collections::HashMap;

pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
    let mut num_map: HashMap<i32, usize> = HashMap::new();

    for (i, &num) in nums.iter().enumerate() {
        let complement = target - num;

        if let Some(&complement_index) = num_map.get(&complement) {
            return vec![complement_index as i32, num];
        }

        num_map.insert(num, i);
    }

    vec![]
}
