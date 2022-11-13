// Definition for singly-linked list.
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    #[allow(unused)]
    fn new(val: i32) -> Self {
        ListNode { next: None, val }
    }
}

struct Solution {}
impl Solution {
    #[allow(unused)]
    pub fn reverse_list(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        Solution::rec_rev(None, head)
    }

    fn rec_rev(prev: Option<Box<ListNode>>, cur: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        match cur {
            Some(mut node) => {
                let next = node.next;
                node.next = prev;
                Solution::rec_rev(Some(node), next)
            }
            None => prev,
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::problems::q0206_reverse_linked_list::ListNode;
    use crate::problems::q0206_reverse_linked_list::Solution;

    #[test]
    fn test() {
        let list = None;
        assert_eq!(Solution::reverse_list(list), None);

        let list = Some(Box::new(ListNode { val: 1, next: None }));
        let expected = Some(Box::new(ListNode { val: 1, next: None }));
        assert_eq!(Solution::reverse_list(list), expected);

        let list = Some(Box::new(ListNode {
            val: 1,
            next: Some(Box::new(ListNode { val: 2, next: None })),
        }));
        let expected = Some(Box::new(ListNode {
            val: 2,
            next: Some(Box::new(ListNode { val: 1, next: None })),
        }));
        assert_eq!(Solution::reverse_list(list), expected);

        let list = Some(Box::new(ListNode {
            val: 1,
            next: Some(Box::new(ListNode {
                val: 2,
                next: Some(Box::new(ListNode { val: 3, next: None })),
            })),
        }));
        let expected = Some(Box::new(ListNode {
            val: 3,
            next: Some(Box::new(ListNode {
                val: 2,
                next: Some(Box::new(ListNode { val: 1, next: None })),
            })),
        }));
        assert_eq!(Solution::reverse_list(list), expected);
    }
}
