pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    pub fn new(val: i32) -> Self {
        Self { val, next: None }
    }
}

pub fn split_list_to_parts(mut head: Option<Box<ListNode>>, k: i32) -> Vec<Option<Box<ListNode>>> {
    let mut len = 0;
    let mut curr = head.as_ref();
    while let Some(node) = curr {
        len += 1;
        curr = node.next.as_ref();
    }

    let base_size = len / k;
    let extra = len % k;

    let mut result: Vec<Option<Box<ListNode>>> = Vec::with_capacity(k as usize);

    for i in 0..k as usize {
        let part_size = base_size + if i < extra as usize { 1 } else { 0 };

        result.push(head.take());

        if part_size > 0 {
            let mut curr = result[i].as_mut().unwrap();
            for _ in 1..part_size {
                if let Some(next) = curr.next.take() {
                    curr.next = Some(next);
                    curr = curr.next.as_mut().unwrap();
                }
            }
            head = curr.next.take();
        }
    }

    result
}
