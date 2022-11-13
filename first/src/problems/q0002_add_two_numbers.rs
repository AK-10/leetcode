// Definition for singly-linked list.
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

    pub fn add_two_numbers(l1: Option<Box<ListNode>>, l2: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut dummy = ListNode::new(0);
        let mut p = &l1;
        let mut q = &l2;
        let mut carry = 0;
        let mut current = &mut dummy;

        while p.is_some() || q.is_some() {
            let x = if let Some(pnode) = p { pnode.val } else { 0 };
            let y = if let Some(qnode) = q { qnode.val } else { 0 };

            let sum = x + y + carry;
            carry = sum / 10;
            current.next = Some(Box::new(ListNode::new(sum % 10)));
            current = current.next.as_mut().unwrap();

            if let Some(pnode) = p {
                p = &pnode.next;
            }
            if let Some(qnode) = q {
                q = &qnode.next;
            }
        }

        if carry > 0 {
            current.next = Some(Box::new(ListNode::new(carry)))
        }

        dummy.next
    }
    // max100桁なので、一度全体を数字に落として計算するのはできない
    // 一桁ずつ計算する必要がある
    //pub fn add_two_numbers(l1: Option<Box<ListNode>>, l2: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    //    // create l1 num, l2 num
    //    let l1_num = Solution::get_num(l1, 0);
    //    let l2_num = Solution::get_num(l2, 0);

    //    // add two num
    //    let answer_num = l1_num + l2_num;

    //    // create listnode
    //    Some(Box::new(Solution::get_listnode(answer_num)))

    //}

    //fn get_num(list: Option<Box<ListNode>>, digit: u32) -> i64 {
    //    match list {
    //        Some(x) => {
    //            // 足した結果がi32を超えることがあるのでi64に拡張する
    //            // i32 max = 2_147_483_647
    //            Solution::get_num(x.next, digit + 1) + x.val as i64 * 10i64.pow(digit)
    //        }
    //        None => 0
    //    }
    //}

    //fn get_listnode(val: i64) -> ListNode {
    //    let node_val = val % 10;
    //    let next_val = val / 10;

    //    ListNode {
    //        next: if next_val != 0 {
    //            Some(Box::new(Solution::get_listnode(next_val)))
    //        } else {
    //            None
    //        },
    //        val: node_val as i32
    //    }
    //}
}


#[cfg(test)]
mod test {
    #[test]
    fn q0002_add_two_numbers() {
        use crate::problems::q0002_add_two_numbers::Solution;
        use crate::problems::q0002_add_two_numbers::ListNode;

        //let l1 = Some(Box::new(ListNode {
        //    val: 2,
        //    next: Some(Box::new(ListNode {
        //        val: 4,
        //        next: Some(Box::new(ListNode {
        //            val: 3,
        //            next: None
        //        }))
        //    }))
        //}));
        //let l2 = Some(Box::new(ListNode {
        //    val: 5,
        //    next: Some(Box::new(ListNode {
        //        val: 6,
        //        next: Some(Box::new(ListNode {
        //            val: 4,
        //            next: None
        //        }))
        //    }))
        //}));
        //let answer = Some(Box::new(ListNode {
        //    val: 7,
        //    next: Some(Box::new(ListNode {
        //        val: 0,
        //        next: Some(Box::new(ListNode {
        //            val: 8,
        //            next: None
        //        }))
        //    }))
        //}));
        //assert_eq!(Solution::add_two_numbers(l1, l2), answer);

        //let l1 = Some(Box::new(ListNode {
        //    val: 0,
        //    next: None
        //}));
        //let l2 = Some(Box::new(ListNode {
        //    val: 0,
        //    next: None
        //}));
        //let answer = Some(Box::new(ListNode {
        //    val: 0,
        //    next: None
        //}));
        //assert_eq!(Solution::add_two_numbers(l1, l2), answer);

        //let l1 = Some(Box::new(ListNode {
        //    val: 9,
        //    next: Some(Box::new(ListNode {
        //        val: 9,
        //        next: Some(Box::new(ListNode {
        //            val: 9,
        //            next: Some(Box::new(ListNode {
        //                val: 9,
        //                next: None
        //            }))
        //        }))
        //    }))
        //}));
        //let l2 = Some(Box::new(ListNode {
        //    val: 9,
        //    next: Some(Box::new(ListNode {
        //        val: 9,
        //        next: Some(Box::new(ListNode {
        //            val: 9,
        //            next: Some(Box::new(ListNode {
        //                val: 9,
        //                next: Some(Box::new(ListNode {
        //                    val: 9,
        //                    next: Some(Box::new(ListNode {
        //                        val: 9,
        //                        next: Some(Box::new(ListNode {
        //                            val: 9,
        //                            next: None
        //                        }))
        //                    }))
        //                }))
        //            }))
        //        }))
        //    }))
        //}));
        //let answer = Some(Box::new(ListNode {
        //    val: 8,
        //    next: Some(Box::new(ListNode {
        //        val: 9,
        //        next: Some(Box::new(ListNode {
        //            val: 9,
        //            next: Some(Box::new(ListNode {
        //                val: 9,
        //                next: Some(Box::new(ListNode {
        //                    val: 0,
        //                    next: Some(Box::new(ListNode {
        //                        val: 0,
        //                        next: Some(Box::new(ListNode {
        //                            val: 0,
        //                            next: Some(Box::new(ListNode {
        //                                val: 1,
        //                                next: None
        //                            }))
        //                        }))
        //                    }))
        //                }))
        //            }))
        //        }))
        //    }))
        //}));
        //assert_eq!(Solution::add_two_numbers(l1, l2), answer);

        let l1 = Some(Box::new(ListNode {
            val: 9,
            next: None
        }));
        let l2 = Some(Box::new(ListNode {
            val: 1,
            next: Some(Box::new(ListNode {
                val: 9,
                next: Some(Box::new(ListNode {
                    val: 9,
                    next: Some(Box::new(ListNode {
                        val: 9,
                        next: Some(Box::new(ListNode {
                            val: 9,
                            next: Some(Box::new(ListNode {
                                val: 9,
                                next: Some(Box::new(ListNode {
                                    val: 9,
                                    next: Some(Box::new(ListNode {
                                        val: 9,
                                        next: Some(Box::new(ListNode {
                                            val: 9,
                                            next: Some(Box::new(ListNode {
                                                val: 9,
                                                next: None
                                            }))
                                        }))
                                    }))
                                }))
                            }))
                        }))
                    }))
                }))
            }))
        }));
        let answer = Some(Box::new(ListNode {
            val: 0,
            next: Some(Box::new(ListNode {
                val: 0,
                next: Some(Box::new(ListNode {
                    val: 0,
                    next: Some(Box::new(ListNode {
                        val: 0,
                        next: Some(Box::new(ListNode {
                            val: 0,
                            next: Some(Box::new(ListNode {
                                val: 0,
                                next: Some(Box::new(ListNode {
                                    val: 0,
                                    next: Some(Box::new(ListNode {
                                        val: 0,
                                        next: Some(Box::new(ListNode {
                                            val: 0,
                                            next: Some(Box::new(ListNode {
                                                val: 0,
                                                next: Some(Box::new(ListNode {
                                                    val: 1,
                                                    next: None
                                                }))
                                            }))
                                        }))
                                    }))
                                }))
                            }))
                        }))
                    }))
                }))
            }))
        }));
        assert_eq!(Solution::add_two_numbers(l1, l2), answer);
    }
}
