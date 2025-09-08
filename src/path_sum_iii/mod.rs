use std::cell::RefCell;
use std::rc::Rc;
use crate::data_structures::TreeNode;
/// # Path Sum III
/// This problem listed on leetcode problems set
/// URL: https://leetcode.com/problems/path-sum-iii/description/
/// # Description:
/// Given the root of a binary tree and an integer targetSum,
/// return the number of paths where the sum of the values along the path equals targetSum.
/// The path does not need to start or end at the root or a leaf,
/// but it must go downwards (i.e., traveling only from parent nodes to child nodes).
pub struct Solution;

impl Solution {
    pub fn path_sum(root: Option<Rc<RefCell<TreeNode>>>, target_sum: i32) -> i32 {
        match root {
            None => 0,
            Some(root) => {
                Self::helper(Some(root.clone()), target_sum)
                 + Self::path_sum(root.borrow().left.clone(), target_sum)
                + Self::path_sum(root.borrow().right.clone(), target_sum)
            }
        }
    }

    fn helper(root: Option<Rc<RefCell<TreeNode>>>, target_sum: i32) -> i32 {
        match root {
            None => 0,
            Some(root) => {
                let mut count = 0;
                let val = root.borrow().val;
                if val == target_sum {
                    count += 1
                }
                if val >= i32::MAX / 10 || val <= i32::MIN / 10 {
                    return 0;
                }
                let left = Self::helper(root.borrow().left.clone(), target_sum - val);
                let right = Self::helper(root.borrow().right.clone(), target_sum - val);
                count + left + right
            }
        }
    }



}

#[cfg(test)]
mod tests{
    use super::*;

    /// # Example Tree 13
    ///```test
    ///                         (10)
    ///                        /    \
    ///                     (5)     (-3)
    ///                    /   \       \
    ///                  (3)   (2)     (11)
    ///                 /  \      \
    ///               (3)  (-2)   (1)
    /// ```
    /// Paths: 3
    #[test]
    fn test1() {
        let root: Rc<RefCell<TreeNode>> = TreeNode::create_tree_14();
        assert_eq!(Solution::path_sum(Some(root), 8), 3)
    }


    /// # Example tree 12
    /// ```test
    ///
    ///                                 (    5     )
    ///                                /            \
    ///                              (4)            (8)
    ///                             /              /   \
    ///                          (11)            (13)   (4)
    ///                         /   \                  /   \
    ///                      (7)    (2)              (5)   (1)
    /// ```
    /// Paths: 3
    #[test]
    fn test2() {
        let root: Rc<RefCell<TreeNode>> = TreeNode::create_tree_12();
        assert_eq!(Solution::path_sum(Some(root), 22), 3)
    }

    /// Example tree 15
    /// ```test
    ///                      (1_000_000_000)
    ///                         /
    ///                 (1_000_000_000)
    ///                     /
    ///             (294_967_296)
    ///                 /
    ///         (1_000_000_000)
    ///             /
    ///     (1_000_000_000)
    /// ```test
    /// Paths: 0, Overflowing
    #[test]
    fn test3() {
        let root: Rc<RefCell<TreeNode>> = TreeNode::create_tree_15();
        assert_eq!(Solution::path_sum(Some(root), 0), 0);
    }
}