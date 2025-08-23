use crate::data_structures::TreeNode;
use std::cell::RefCell;
use std::rc::Rc;

/// # Maximum Depth of Binary Tree
/// This problem is listed on leet code problems set
/// URL: https://leetcode.com/problems/maximum-depth-of-binary-tree/description/
///
/// # Description:
/// Given the root of a binary tree, return its maximum depth.
///
/// A binary tree's maximum depth is the number of nodes along the longest path
/// from the root node down to the farthest leaf node.

pub struct Solution;

impl Solution {

    /// This is the classic DFS implementation
    pub fn max_depth(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        if root.is_none() {
            // root must not be none, if it is none return 0
            return 0i32;
        }
        let root = root.unwrap(); // Just to avoid using unwrap when getting left node and right node
        let depth_left = Self::max_depth(root.borrow().left.clone()); // Left node max depth
        let depth_right = Self::max_depth(root.borrow().right.clone()); // Right node max depth
        1i32 + depth_left.max(depth_right) // returns 1 plus max(children's max depth)
    }
}


