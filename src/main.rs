//use leetcode::problems::q0001_two_sum;

use leetcode::problems::q0083_remove_duplicates_from_sorted_list::{ListNode, Solution};
fn main() {
    println!("Hello, world!");
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
}
