use std::cell::RefCell;
use std::rc::Rc;
use crate::data_structures::TreeNode;
/// Path sum I
/// This problem is listed on leetcode problems set
/// URL: https://leetcode.com/problems/path-sum/description/
/// # Description:
/// Given the root of a binary tree and an integer targetSum,
/// return true if the tree has a root-to-leaf path such that
/// adding up all the values along the path equals targetSum.
/// A leaf is a node with no children.
pub struct Solution;

impl Solution {

    pub fn has_path_sum(root: Option<Rc<RefCell<TreeNode>>>, sum: i32) -> bool {
        if root.is_none() {
            return false
        }
        let root = root.unwrap();
        let val = root.borrow().val;
        if root.borrow().left.is_none() && root.borrow().right.is_none() {
            return val == sum;
        }
        let left_sum = Self::has_path_sum(root.borrow().left.clone(), sum - val);
        let right_sum = Self::has_path_sum(root.borrow().right.clone(), sum - val);
        left_sum || right_sum
    }

}


#[cfg(test)]
mod tests {
    use super::*;
    /// # Example tree 9
    ///
    ///                                 (    5     )
    ///                                /            \
    ///                              (4)            (8)
    ///                             /              /   \
    ///                          (11)            (13)   (4)
    ///                         /   \                     \
    ///                      (7)    (2)                   (1)
    ///
    /// Has path sum: true
    #[test]
    fn test1() {
        let root: Rc<RefCell<TreeNode>> = TreeNode::create_tree_9();
        assert!(Solution::has_path_sum(Some(root), 22))
    }


    /// # Example test 1
    ///                                   (1)
    ///                                  /   \
    ///                                (2)   (3)
    ///
    /// Has path sum: false
    #[test]
    fn test2() {
        let root: Rc<RefCell<TreeNode>> = TreeNode::create_tree_1();
        assert!(!Solution::has_path_sum(Some(root), 0))
    }

    /// # Example tree 8
    ///
    ///                                 (       1       )
    ///                                /                \
    ///                             (10)                (15)
    ///                            /    \               /   \
    ///                         (-5)    (6)           (2)   (-2)
    ///                                /   \                   \
    ///                             (7)    (-20)              (30)
    /// Has path sum: true
    #[test]
    fn test3() {
        let root: Rc<RefCell<TreeNode>> = TreeNode::create_tree_8();
        assert!(Solution::has_path_sum(Some(root), 44))
    }

    /// # Example tree 6:
    ///                         (2)
    ///                        /
    ///                     (-1)
    /// Has path sum: false
    #[test]
    fn test4() {
        let root: Rc<RefCell<TreeNode>> = TreeNode::create_tree_6();
        assert!(!Solution::has_path_sum(Some(root), 3))
    }

    /// # Example tree 10
    ///
    ///                         (10)
    ///                        /   \
    ///                     (2)    (11)
    ///                    /
    ///                  (0)
    ///
    /// Has path sum: true
    #[test]
    fn test5() {
        let root: Rc<RefCell<TreeNode>> = TreeNode::create_tree_10();
        assert!(Solution::has_path_sum(Some(root), 12))
    }

    /// # Example tree 11
    ///                             (-2)
    ///                                \
    ///                                (-3)
    ///
    /// Has path sum: true
    #[test]
    fn test6(){
        let root: Rc<RefCell<TreeNode>> = TreeNode::create_tree_11();
        assert!(Solution::has_path_sum(Some(root), -5))
    }
}