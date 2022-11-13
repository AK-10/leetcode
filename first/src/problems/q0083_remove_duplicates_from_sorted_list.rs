//Definition for singly-linked list.
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>
}

impl ListNode {
    #[inline]
    fn new(val: i32) -> Self {
        ListNode {
            next: None,
            val
        }
    }
}

pub struct Solution {}
impl Solution {
    fn remove(node: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        match node {
            Some(node) => {
                if let Some(next_node) = &node.next {
                    if node.val == next_node.val {
                        Solution::remove(node.next)
                    } else {
                        Some(Box::new(ListNode{ val: node.val, next: Solution::remove(node.next) }))
                    }
                } else {
                    Some(Box::new(ListNode::new(node.val)))
                }
            }
            None => None
        }
    }

    pub fn delete_duplicates(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        match head {
            Some(_) => {
                Solution::remove(head)
                //let mut res = ListNode::new(101);
                //let mut res_current = &mut res;
                //let mut current = &head;
                //while let Some(nd) = current {
                //    if let Some(nd_next) = &nd.next {
                //        if nd_next.val != nd.val {
                //            res_current.next = Some(Box::new(ListNode::new(nd.val)));
                //            res_current = res_current.next.as_mut().unwrap();
                //        }
                //    } else {
                //        res_current.next = Some(Box::new(ListNode::new(nd.val)));
                //        res_current = res_current.next.as_mut().unwrap();
                //    }

                //    current = &nd.next;
                //}

                //res.next
            }
            None => head
        }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn q0083_remove_duplicates_test() {
        use crate::problems::q0083_remove_duplicates_from_sorted_list::{ListNode, Solution};

        let list = Some(Box::new(ListNode {
            val: 1,
            next: Some(Box::new(ListNode {
                val: 1,
                next: Some(Box::new(ListNode {
                    val: 2,
                    next: None
                }))
            }))
        }));
        let expected = Some(Box::new(ListNode {
            val: 1,
            next: Some(Box::new(ListNode {
                val: 2,
                next: None
            }))
        }));
        assert_eq!(Solution::delete_duplicates(list), expected);

        let list = Some(Box::new(ListNode {
            val: 1,
            next: Some(Box::new(ListNode {
                val: 1,
                next: Some(Box::new(ListNode {
                    val: 2,
                    next: Some(Box::new(ListNode {
                        val: 3,
                        next: Some(Box::new(ListNode {
                            val: 3,
                            next: None
                        }))
                    }))
                }))
            }))
        }));
        let expected = Some(Box::new(ListNode {
            val: 1,
            next: Some(Box::new(ListNode {
                val: 2,
                next: Some(Box::new(ListNode {
                    val: 3,
                    next: None
                }))
            }))
        }));
        assert_eq!(Solution::delete_duplicates(list), expected);

        assert_eq!(Solution::delete_duplicates(None), None);
    }
}
