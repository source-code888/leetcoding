//! # House Robber III
//! ## Problem description:
//! The thief has found himself a new place for his thievery again.
//! There is only one entrance to this area, called root.
//! Besides the root, each house has one and only one parent house.
//! After a tour, the smart thief realized that all houses in this place form a binary tree.
//! It will automatically contact the police if two directly-linked houses were broken into on the same night.
//! Given the root of the binary tree, return the maximum amount of money the thief can rob without
//! alerting the police.
//! ## Approach
//! DFS, I tried level order traversal first xd
//! After taking a closed look at this problem, I realized that we can do the next:
//! - Rob including the root and rob excluding the root
//! - If **root** is leaf, we should return its **value**.
//! - If we rob including **root**, and it has child nodes, then we have to
//! rob excluding the **root** of them.
//! - If we rob excluding **root**, and it has child nodes, so we can rob to
//! its child nodes in four ways and keep the **max value**.
//!     - **Option 1**: Rob including **left root** and **right root**.
//!     - **Option 2**: Rob including **left root** and excluding **right root**.
//!     - **Option 3**: Rob excluding **left root** and including **right root**.
//!     - **Option 4**: Rob excluding **both** roots.
//! - Do not forget to keep the **max value** between those four combinations.
use crate::data_structures::TreeNode;
use std::cell::RefCell;
use std::rc::Rc;
pub struct Solution;

impl Solution {
    pub fn rob(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        let r = Self::helper(root);
        r.0.max(r.1)
    }

    fn helper(root: Option<Rc<RefCell<TreeNode>>>) -> (i32, i32) {
        match root {
            None => (0, 0),
            Some(root) => {
                let val = root.borrow().val;
                if root.borrow().left.is_none() && root.borrow().right.is_none() {
                    return (val, 0i32)
                }
                let (includes_left_root, excludes_left_root) = Self::helper(root.borrow().left.clone());
                let (includes_right_root, excludes_right_root) = Self::helper(root.borrow().right.clone());
                let excludes_root = (includes_left_root  + includes_right_root).max(includes_left_root + excludes_right_root)
                    .max(excludes_left_root + includes_right_root).max(excludes_left_root + excludes_right_root);
                (val + excludes_left_root + excludes_right_root, excludes_root)
            }
        }
    }

}
pub mod tree {
    use super::*;

    /// # Example 1
    ///```text
    ///                     (3)
    ///                    /   \
    ///                 (2)    (3)
    ///                    \     \
    ///                    (3)   (1)
    ///```
    pub fn create_tree_1() -> Option<Rc<RefCell<TreeNode>>> {
        let root: Rc<RefCell<TreeNode>> = Rc::new(RefCell::new(TreeNode::new(3)));
        let left: Rc<RefCell<TreeNode>> = Rc::new(RefCell::new(TreeNode::new(2)));
        left.borrow_mut().right = Some(Rc::new(RefCell::new(TreeNode::new(3))));
        root.borrow_mut().left = Some(left);
        let right: Rc<RefCell<TreeNode>> = Rc::new(RefCell::new(TreeNode::new(3)));
        right.borrow_mut().right = Some(Rc::new(RefCell::new(TreeNode::new(1))));
        root.borrow_mut().right = Some(right);
        Some(root)
    }

    /// # Example 2
    ///```text
    ///                             (3)
    ///                            /   \
    ///                         (4)    (5)
    ///                        /   \     \
    ///                     (1)    (3)   (1)
    /// ```
    pub fn create_tree_2() -> Option<Rc<RefCell<TreeNode>>> {
        let root: Rc<RefCell<TreeNode>> = Rc::new(RefCell::new(TreeNode::new(3)));
        let left: Rc<RefCell<TreeNode>> = Rc::new(RefCell::new(TreeNode::new(4)));
        left.borrow_mut().left = Some(Rc::new(RefCell::new(TreeNode::new(1))));
        left.borrow_mut().right = Some(Rc::new(RefCell::new(TreeNode::new(3))));
        root.borrow_mut().left = Some(left);
        let right: Rc<RefCell<TreeNode>> = Rc::new(RefCell::new(TreeNode::new(5)));
        right.borrow_mut().right = Some(Rc::new(RefCell::new(TreeNode::new(1))));
        root.borrow_mut().right = Some(right);
        Some(root)
    }

    /// # Example 3
    ///```text
    ///                                 (   1   )
    ///                                /         \
    ///                             (9)          (2)
    ///                            /   \            \
    ///                         (2)    (3)          (4)
    ///                        /  \                    \
    ///                     (6)   (3)                  (7)
    ///                             \
    ///                             (1)
    /// ```
    pub fn create_tree_3() -> Option<Rc<RefCell<TreeNode>>> {
        let root: Rc<RefCell<TreeNode>> = Rc::new(RefCell::new(TreeNode::new(1)));
        let left: Rc<RefCell<TreeNode>> = Rc::new(RefCell::new(TreeNode::new(9)));
        left.borrow_mut().right = Some(Rc::new(RefCell::new(TreeNode::new(3))));
        let left_l: Rc<RefCell<TreeNode>> = Rc::new(RefCell::new(TreeNode::new(2)));
        left_l.borrow_mut().left = Some(Rc::new(RefCell::new(TreeNode::new(6))));
        let left_r: Rc<RefCell<TreeNode>> = Rc::new(RefCell::new(TreeNode::new(3)));
        left_r.borrow_mut().right = Some(Rc::new(RefCell::new(TreeNode::new(1))));
        left_l.borrow_mut().right = Some(left_r);
        left.borrow_mut().left = Some(left_l);
        root.borrow_mut().left = Some(left);
        let right: Rc<RefCell<TreeNode>> = Rc::new(RefCell::new(TreeNode::new(2)));
        let right_r: Rc<RefCell<TreeNode>> = Rc::new(RefCell::new(TreeNode::new(4)));
        right_r.borrow_mut().right = Some(Rc::new(RefCell::new(TreeNode::new(7))));
        right.borrow_mut().right = Some(right_r);
        root.borrow_mut().right = Some(right);
        Some(root)
    }

    /// # Example 4
    ///```text
    ///                             (15)
    ///                            /   \
    ///                         (2)    (3)
    ///                        /          \
    ///                     (1)           (7)
    /// ```
    pub fn create_tree_4() -> Option<Rc<RefCell<TreeNode>>> {
        let root: Rc<RefCell<TreeNode>> = Rc::new(RefCell::new(TreeNode::new(15)));
        let left: Rc<RefCell<TreeNode>> = Rc::new(RefCell::new(TreeNode::new(2)));
        left.borrow_mut().left = Some(Rc::new(RefCell::new(TreeNode::new(1))));
        root.borrow_mut().left = Some(left);
        let right: Rc<RefCell<TreeNode>> = Rc::new(RefCell::new(TreeNode::new(3)));
        right.borrow_mut().right = Some(Rc::new(RefCell::new(TreeNode::new(7))));
        root.borrow_mut().right = Some(right);
        Some(root)
    }

    /// # Example 5
    ///```text
    ///                             (15)
    ///                            /   \
    ///                         (2)    (3)
    ///                        /          \
    ///                     (1)           (7)
    ///                        \         /
    ///                        (9)    (10)
    /// ```
    pub fn create_tree_5() -> Option<Rc<RefCell<TreeNode>>> {
        let root: Rc<RefCell<TreeNode>> = Rc::new(RefCell::new(TreeNode::new(15)));
        let left: Rc<RefCell<TreeNode>> = Rc::new(RefCell::new(TreeNode::new(2)));
        let left_l: Rc<RefCell<TreeNode>> = Rc::new(RefCell::new(TreeNode::new(1)));
        left_l.borrow_mut().right = Some(Rc::new(RefCell::new(TreeNode::new(9))));
        left.borrow_mut().left = Some(left_l);
        root.borrow_mut().left = Some(left);
        let right: Rc<RefCell<TreeNode>> = Rc::new(RefCell::new(TreeNode::new(3)));
        let right_r: Rc<RefCell<TreeNode>> = Rc::new(RefCell::new(TreeNode::new(7)));
        right_r.borrow_mut().left = Some(Rc::new(RefCell::new(TreeNode::new(10))));
        right.borrow_mut().right = Some(right_r);
        root.borrow_mut().right = Some(right);
        Some(root)
    }

    /// # Example 6
    /// ```text
    ///                         (4)
    ///                        /
    ///                     (1)
    ///                    /
    ///                 (2)
    ///                /
    ///             (3)
    /// ```
    pub fn create_tree_6() -> Option<Rc<RefCell<TreeNode>>> {
        let root: Rc<RefCell<TreeNode>> = Rc::new(RefCell::new(TreeNode::new(4)));
        let left: Rc<RefCell<TreeNode>> = Rc::new(RefCell::new(TreeNode::new(1)));
        let left_l: Rc<RefCell<TreeNode>> = Rc::new(RefCell::new(TreeNode::new(2)));
        left_l.borrow_mut().left = Some(Rc::new(RefCell::new(TreeNode::new(3))));
        left.borrow_mut().left = Some(left_l);
        root.borrow_mut().left = Some(left);
        Some(root)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test1() {
        assert_eq!(Solution::rob(tree::create_tree_1()), 7);
    }

    #[test]
    fn test2() {
        assert_eq!(Solution::rob(tree::create_tree_2()), 9);
    }

    #[test]
    fn test3() {
        assert_eq!(Solution::rob(tree::create_tree_3()), 27);
    }

    #[test]
    fn test4() {
        assert_eq!(Solution::rob(tree::create_tree_4()), 23);
    }

    #[test]
    fn test5() {
        assert_eq!(Solution::rob(tree::create_tree_5()), 34);
    }

    #[test]
    fn test6() {
        assert_eq!(Solution::rob(tree::create_tree_6()), 7);
    }
}
