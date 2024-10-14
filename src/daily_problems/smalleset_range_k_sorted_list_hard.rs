use std::collections::BinaryHeap;

#[derive(Copy, Clone, Eq, PartialEq)]
struct Element {
    value: i32,
    list_index: usize,
    position: usize,
}

impl Ord for Element {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other.value.cmp(&self.value)
    }
}

impl PartialOrd for Element {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

fn smallest_range(nums: Vec<Vec<i32>>) -> Vec<i32> {
    let k = nums.len();
    let mut heap = BinaryHeap::new();
    let mut max = i32::MIN;

    for (i, list) in nums.iter().enumerate() {
        if let Some(&value) = list.first() {
            heap.push(Element {
                value,
                list_index: i,
                position: 0,
            });
            max = max.max(value);
        }
    }

    let mut range = vec![0, i32::MAX];

    while heap.len() == k {
        if let Some(Element {
            value: min,
            list_index,
            position,
        }) = heap.pop()
        {
            if max - min < range[1] - range[0]
                || (max - min == range[1] - range[0] && min < range[0])
            {
                range[0] = min;
                range[1] = max;
            }

            if position + 1 < nums[list_index].len() {
                let next_value = nums[list_index][position + 1];
                heap.push(Element {
                    value: next_value,
                    list_index,
                    position: position + 1,
                });
                max = max.max(next_value);
            }
        }
    }
    range
}

#[test]
fn it_works() {
    let nums = vec![
        vec![4, 10, 15, 24, 26],
        vec![0, 9, 12, 20],
        vec![5, 18, 22, 30],
    ];

    let result = smallest_range(nums);
    println!("Smallest range :[{} , {}]", result[0], result[1]);
}
