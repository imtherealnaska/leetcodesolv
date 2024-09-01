use std::collections::HashMap;

pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
    let mut num_map: HashMap<i32, usize> = HashMap::new();

    for (i, &num) in nums.iter().enumerate() {
        let complement = target - num;

        if let Some(&complement_index) = num_map.get(&complement) {
            return vec![complement_index , num];
        }

        num_map.insert(num, i);
    }

    vec![]
}

pub fn no_hashmap_two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
    let mut ret_vec: Vec<i32> = Vec::new();
    'loopy: for (i, &num) in nums.iter().enumerate() {
        let complement = target - num;
        if let Some(pos_) = linear_search(&nums[i + 1..], &complement) {
            ret_vec = vec![i as i32, (i + 1 + pos_) as i32];
            break 'loopy;
        } else {
            continue;
        }
    }
    ret_vec
}

fn linear_search<T>(ar: &[T], element: &T) -> Option<usize>
where
    T: PartialOrd,
{
    for (pos, val) in ar.iter().enumerate() {
        if val == element {
            return Some(pos);
        }
    }
    None
}
