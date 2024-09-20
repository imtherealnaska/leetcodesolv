#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    pub fn new(val: i32) -> Self {
        ListNode { next: None, val }
    }
}

pub struct Solution;

impl Solution {
    pub fn spiral_matrix(m: i32, n: i32, head: Option<Box<ListNode>>) -> Vec<Vec<i32>> {
        let (m, n) = (m as usize, n as usize);
        let mut matrix = vec![vec![-1; n]; m];
        let mut current = head;
        let mut row = 0;
        let mut col = 0;
        let mut top = 0;
        let mut bottom = m - 1;
        let mut left = 0;
        let mut right = n - 1;

        while let Some(node) = current {
            matrix[row][col] = node.val;

            if row == top && col < right {
                col += 1;
            } else if col == right && row < bottom {
                row += 1;
            } else if row == bottom && col > left {
                col -= 1;
            } else if col == left && row > top {
                row -= 1;
            }

            if row == top && col == left {
                top += 1;
                bottom -= 1;
                left += 1;
                right -= 1;
                row = top;
                col = left;
            }

            current = node.next;
            if row >= m || col >= n {
                break;
            }
        }
        matrix
    }
}
