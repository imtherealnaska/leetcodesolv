use std::collections::HashSet;

/// Think of this as a general problem for all
/// consecutive seq numbers with some property .
pub fn longest_consecutive(nums: Vec<i32>) -> i32 {
    if nums.is_empty() {
        return 0;
    }

    let num_set: HashSet<i32> = nums.into_iter().collect();
    let mut longest_streak = 1;

    for &num in &num_set {
        if !num_set.contains(&(num - 1)) {
            let mut current_num = num;
            let mut current_streak = 1;

            while num_set.contains(&(current_num + 1)) {
                current_num += 1;
                current_streak += 1;
            }

            longest_streak = longest_streak.max(current_streak);
        }
    }
    longest_streak
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_empty() {
        assert_eq!(longest_consecutive(vec![]), 0);
    }

    #[test]
    fn test_single_element() {
        assert_eq!(longest_consecutive(vec![1]), 1);
    }

    #[test]
    fn test_no_sequence() {
        assert_eq!(longest_consecutive(vec![2, 4, 6, 8]), 1);
    }

    #[test]
    fn test_basic_sequence() {
        assert_eq!(longest_consecutive(vec![1, 2, 3, 4]), 4);
    }

    #[test]
    fn test_unsorted_sequence() {
        assert_eq!(longest_consecutive(vec![100, 4, 200, 1, 3, 2]), 4);
    }

    #[test]
    fn test_multiple_sequences() {
        assert_eq!(longest_consecutive(vec![0, 3, 7, 2, 5, 8, 4, 6, 1]), 9);
    }

    #[test]
    fn test_duplicate_numbers() {
        assert_eq!(longest_consecutive(vec![1, 2, 2, 3, 4, 4, 5]), 5);
    }

    #[test]
    fn test_negative_numbers() {
        assert_eq!(longest_consecutive(vec![-5, -4, -3, -2, -1, 0, 1]), 7);
    }
}
