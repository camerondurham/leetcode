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
    pub fn get_all_elements(
        root1: Option<Rc<RefCell<TreeNode>>>,
        root2: Option<Rc<RefCell<TreeNode>>>,
    ) -> Vec<i32> {
        let mut ans: Vec<i32> = Vec::new();
        if let Some(r) = root1 {
            Solution::traverse(Some(&r), &mut ans);
        }

        if let Some(r) = root2 {
            Solution::traverse(Some(&r), &mut ans);
        }

        ans.sort();
        ans
    }

    fn traverse(root: Option<&Rc<RefCell<TreeNode>>>, vec: &mut Vec<i32>) -> () {
        if let Some(node) = root {
            Solution::traverse(node.borrow().left.as_ref(), vec);
            vec.push(node.borrow().val);
            Solution::traverse(node.borrow().right.as_ref(), vec);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ex1() {
        assert_eq!(2 + 2, 4);
    }
}
