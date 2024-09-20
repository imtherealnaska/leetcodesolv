use std::collections::HashSet;

pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    fn new(val: i32) -> Self {
        ListNode { val, next: None }
    }
}

pub fn modified_list(nums: Vec<i32>, head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    let num_set: HashSet<i32> = nums.into_iter().collect();

    // Create a dummy head to handle the case where the head needs to be removed
    let mut dummy = Box::new(ListNode::new(0));
    dummy.next = head;

    let mut current = &mut dummy;

    while let Some(next) = current.next.take() {
        if num_set.contains(&next.val) {
            // Skip this node
            current.next = next.next;
        } else {
            // Move to the next node
            current.next = Some(next);
            current = current.next.as_mut().unwrap();
        }
    }

    dummy.next
}
