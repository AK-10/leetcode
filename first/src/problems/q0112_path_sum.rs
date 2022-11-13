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
    pub fn has_path_sum(root: Option<Rc<RefCell<TreeNode>>>, target_sum: i32) -> bool {
        match root {
            Some(node) => {
                let node = node.borrow();
                let rest = target_sum - node.val;
                if Self::is_leaf(&node) {
                    rest == 0
                } else {
                    Self::has_path_sum(node.left.clone(), rest) || Self::has_path_sum(node.right.clone(), rest)
                }
            }
            None => false,
        }
    }

    fn is_leaf(node: &TreeNode) -> bool {
        match node {
            TreeNode { val: _, left: None, right: None } => true,
            _ => false
        }
    }
}

#[cfg(test)]
mod tests {
    // データ作るのが面倒だったのでやらない
}
