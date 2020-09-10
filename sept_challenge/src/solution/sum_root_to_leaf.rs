struct Solution {}
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
impl Solution {
    pub fn sum_root_to_leaf(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        let mut paths: Vec<i32> = Vec::new();
        Self::traverse(0, &root, &mut paths);
        let sum: i32 = paths.iter().sum();
        sum
    }

    fn traverse(prefix: i32, root: &Option<Rc<RefCell<TreeNode>>>, paths: &mut Vec<i32>) {
        if let Some(node) = root {
            let cur_prefix = (prefix << 1) | node.borrow().val;
            let r = node.borrow();
            if r.left.is_none() && r.right.is_none() {
                paths.push(cur_prefix);
            } else {
                Self::traverse(cur_prefix, &r.left, paths);
                Self::traverse(cur_prefix, &r.right, paths);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ex1() {}
}
