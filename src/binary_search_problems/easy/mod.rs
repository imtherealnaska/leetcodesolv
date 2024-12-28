pub mod count_complete_tree_nodes;
pub mod sq_root;

pub fn search_insert(nums: Vec<i32>, target: i32) -> i32 {
    // preconditions :sorted array of numbers .
    let mut left = 0;
    let mut right = nums.len();

    while left < right {
        let mid = left + ((right - left) >> 1);

        if nums[mid] > target {
            left = mid + 1;
        } else {
            right = mid;
        }
    }
    // postcondition : Find the target element and return its index
    left as i32
}
