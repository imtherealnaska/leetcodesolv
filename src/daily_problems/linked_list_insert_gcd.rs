use crate::daily_problems::linked_list_binary::ListNode;

// Definition for singly-linked list.
// #[derive(PartialEq, Eq, Clone, Debug)]
// pub struct ListNode {
//   pub val: i32,
//   pub next: Option<Box<ListNode>>
// }
//
// impl ListNode {
//   #[inline]
//   fn new(val: i32) -> Self {
//     ListNode {
//       next: None,
//       val
//     }
//   }
// }
pub struct Solution;

impl Solution {
    pub fn insert_greatest_common_divisors(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        head.as_ref()?;

        let mut new_head = Some(Box::new(ListNode::new(0)));
        let mut tail = new_head.as_mut().unwrap();
        let mut current = head;

        while let Some(node) = current {
            tail.next = Some(Box::new(ListNode::new(node.val)));
            tail = tail.next.as_mut().unwrap();

            if let Some(next_node) = node.next.as_ref() {
                let gcd_val = Self::gcd(node.val, next_node.val);
                tail.next = Some(Box::new(ListNode::new(gcd_val)));
                tail = tail.next.as_mut().unwrap();
            }

            current = node.next;
        }

        new_head.unwrap().next
    }

    pub fn gcd(mut a: i32, mut b: i32) -> i32 {
        while b != 0 {
            let temp = b;
            b = a % b;
            a = temp;
        }
        a.abs()
    }
}
