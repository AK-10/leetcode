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

struct Solution;
impl Solution {
    pub fn merge_trees(
        root1: Option<Rc<RefCell<TreeNode>>>,
        root2: Option<Rc<RefCell<TreeNode>>>,
    ) -> Option<Rc<RefCell<TreeNode>>> {
        Solution::ref_merge_trees(&root1, &root2)
    }

    pub fn ref_merge_trees(
        root1: &Option<Rc<RefCell<TreeNode>>>,
        root2: &Option<Rc<RefCell<TreeNode>>>,
    ) -> Option<Rc<RefCell<TreeNode>>> {
        match (root1, root2) {
            (Some(t1), Some(t2)) => {
                let val = t1.borrow().val + t2.borrow().val;
                let left = Solution::ref_merge_trees(&t1.borrow().left, &t2.borrow().left);
                let right = Solution::ref_merge_trees(&t1.borrow().right, &t2.borrow().right);

                Some(Rc::new(RefCell::new(TreeNode { val, left, right })))
            }
            (Some(t), None) => {
                let val = t.borrow().val;
                let left = Solution::ref_merge_trees(&t.borrow().left, &None);
                let right = Solution::ref_merge_trees(&None, &t.borrow().right);

                Some(Rc::new(RefCell::new(TreeNode { val, left, right })))
            }
            (None, Some(t)) => {
                let val = t.borrow().val;
                let left = Solution::ref_merge_trees(&t.borrow().left, &None);
                let right = Solution::ref_merge_trees(&None, &t.borrow().right);

                Some(Rc::new(RefCell::new(TreeNode { val, left, right })))
            }
            (None, None) => None,
        }
    }
}

#[cfg(test)]
mod tests {
    // データ作るのが面倒だったのでやらない
}
