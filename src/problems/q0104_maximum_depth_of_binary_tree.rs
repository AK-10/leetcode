pub struct Solution {}
use std::cell::RefCell;
use std::rc::Rc;

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

impl Solution {
    pub fn max_depth(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        Self::max_depth_by_ref(&root)
    }

    fn max_depth_by_ref(root: &Option<Rc<RefCell<TreeNode>>>) -> i32 {
        match root {
            Some(node) => {
                let node = node.borrow();
                let left_depth = Solution::max_depth_by_ref(&node.left);
                let right_depth = Solution::max_depth_by_ref(&node.right);

                1 + left_depth.max(right_depth)
            }
            None => 0,
        }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test() {
        use crate::problems::q0104_maximum_depth_of_binary_tree::{Solution, TreeNode};
        use std::cell::RefCell;
        use std::rc::Rc;

        let input = None;
        let expected = 0;
        assert_eq!(Solution::max_depth(input), expected);

        let input = Some(Rc::new(RefCell::new(TreeNode {
            val: 3,
            left: Some(Rc::new(RefCell::new(TreeNode {
                val: 9,
                left: None,
                right: None,
            }))),
            right: Some(Rc::new(RefCell::new(TreeNode {
                val: 20,
                left: Some(Rc::new(RefCell::new(TreeNode {
                    val: 15,
                    left: None,
                    right: None,
                }))),
                right: Some(Rc::new(RefCell::new(TreeNode {
                    val: 7,
                    left: None,
                    right: None,
                }))),
            }))),
        })));
        // Some(Rc::new(RefCell::new(TreeNode { val: 20, left: Some(Rc::new(RefCell::new(TreeNode { val: 15, left: None, right: None }))), right:  Some(Rc::new(RefCell::new(TreeNode { val: 7, left: None, right: None })))})))
        let expected = 3;
        assert_eq!(Solution::max_depth(input), expected);

        let input = Some(Rc::new(RefCell::new(TreeNode {
            val: 1,
            left: None,
            right: Some(Rc::new(RefCell::new(TreeNode {
                val: 2,
                left: None,
                right: None,
            }))),
        })));
        let expected = 2;
        assert_eq!(Solution::max_depth(input), expected);
    }
}
