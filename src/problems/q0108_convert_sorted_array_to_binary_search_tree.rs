// Definition for a binary tree node.
#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Rc<RefCell<TreeNode>>>,
    pub right: Option<Rc<RefCell<TreeNode>>>,
}

impl TreeNode {
    #[inline]
    pub fn new(val: i32) -> Self {
        TreeNode {
            val,
            left: None,
            right: None,
        }
    }
}
use std::cell::RefCell;
use std::rc::Rc;

pub struct Solution {}
impl Solution {
    pub fn sorted_array_to_bst(nums: Vec<i32>) -> Option<Rc<RefCell<TreeNode>>> {

        match Self::split_vec(nums) {
            (Some(lower), Some(middle), Some(higher)) => {
                let left = Self::sorted_array_to_bst(lower);
                let right = Self::sorted_array_to_bst(higher);
                Some(Rc::new(RefCell::new(TreeNode {
                    val: middle,
                    left,
                    right,
                })))
            }
            (Some(lower), Some(middle), None) => {
                let left = Self::sorted_array_to_bst(lower);
                Some(Rc::new(RefCell::new(TreeNode {
                    val: middle,
                    left,
                    right: None,
                })))
            }
            (None, Some(middle), None) => Some(Rc::new(RefCell::new(TreeNode {
                val: middle,
                left: None,
                right: None,
            }))),
            (None, None, None) => None,
            _ => unreachable!(""),
        }
    }

    fn split_vec(nums: Vec<i32>) -> (Option<Vec<i32>>, Option<i32>, Option<Vec<i32>>) {
        if !(nums.len() > 0) {
            return (None, None, None);
        }

        let middle: usize = nums.len() / 2;
        (
            Some(nums[0..middle].to_vec()),
            Some(nums[middle]),
            Some(nums[(middle + 1)..].to_vec()),
        )
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test() {
        //use crate::problems::q0108_convert_sorted_array_to_binary_search_tree::Solution;

        //let input: Vec<i32> = vec![-10, -3, 0, 5, 9];
        //let expected =
        //assert_eq!()
    }
}
