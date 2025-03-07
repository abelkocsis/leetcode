pub fn run() {
    //println!("{}", Solution::min_changes("0000".to_string()));
}

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
    pub fn invert_tree(root: Option<Rc<RefCell<TreeNode>>>) -> Option<Rc<RefCell<TreeNode>>> {
        if let Some(r) = root {
            let tmp = r.borrow();
            let mut tr_node = TreeNode::new(tmp.val);
            tr_node.right = Solution::invert_tree(tmp.left.clone());
            tr_node.left = Solution::invert_tree(tmp.right.clone());
            Some(Rc::new(RefCell::new(tr_node)))
        } else {
            None
        }
    }
}
