use crate::data_structures::TreeNode;
use std::cell::RefCell;
use std::rc::Rc;

/// # Sum of left leaves
/// This problem is listed on leetcode problems set
/// URl: https://leetcode.com/problems/sum-of-left-leaves/description/
/// # Description:
/// Given the root of a binary tree, return the sum of all left leaves.
/// A leaf is a node with no children.
/// A left leaf is a leaf that is the left child of another node.

pub struct Solution;

impl Solution {
    pub fn sum_of_left_leaves(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        Self::helper(root, false)
    }

    pub fn helper(root: Option<Rc<RefCell<TreeNode>>>, is_left: bool) -> i32 {
        if root.is_none() {
            return 0;
        }
        let root: Rc<RefCell<TreeNode>> = root.unwrap();

        if root.borrow().left.is_none() && root.borrow().right.is_none() && is_left {
            return root.borrow().val;
        }

        let left_sum: i32 = Self::helper(root.borrow().left.clone(), true);
        let right_sum: i32 = Self::helper(root.borrow().right.clone(), false);

        left_sum + right_sum
    }
}

#[cfg(test)]
mod tests {
    use crate::data_structures::TreeNode;
    use crate::soll::Solution;
    use std::cell::RefCell;
    use std::rc::Rc;

    /// # Example test 1
    ///
    ///                     (8)
    ///
    /// # Left leaves sum = 0, Since it's not a left node
    #[test]
    pub fn test_1() {
        let root: Rc<RefCell<TreeNode>> = Rc::new(RefCell::new(TreeNode::new(8)));
        println!("root: {:?}", root);
        let sum = Solution::sum_of_left_leaves(Some(root));
        println!("Left leaves sum: {sum}");
        assert_eq!(sum, 0)
    }

    /// # Example test 3
    ///
    ///                             (-10)
    ///                            /     \
    ///                         (9 )    (20)
    ///                                /    \
    ///                             (15)    (7)
    /// # Left leaves sum = 24
    #[test]
    pub fn test_2() {
        let root: Rc<RefCell<TreeNode>> = TreeNode::create_tree_2();
        println!("root: {:?}", root);
        let sum = Solution::sum_of_left_leaves(Some(root));
        println!("Left leaves sum: {sum}");
        assert_eq!(sum, 24)
    }

    /// # Example test 5
    ///
    ///                            (  1   )
    ///                           /       \
    ///                         (-2)      (-3)
    ///                        /  \      /
    ///                     (1)  (3)  (-2)
    ///                    /
    ///                  (-1)
    /// # Left leaves sum: -3
    #[test]
    pub fn test_5() {
        let root: Rc<RefCell<TreeNode>> = TreeNode::create_tree_5();
        println!("root: {:?}", root);
        let sum = Solution::sum_of_left_leaves(Some(root));
        println!("Left leaves sum: {sum}");
        assert_eq!(sum, -3)
    }

    /// # Example test 8
    ///
    ///                                 (       1       )
    ///                                /                \
    ///                             (10)                (15)
    ///                            /    \               /   \
    ///                         (-5)    (6)           (2)   (-2)
    ///                                /   \                   \
    ///                             (7)    (-20)              (30)
    /// Left leaves sum:  -5 + 7 + 2 = 4
    #[test]
    pub fn test_8() {
        let root: Rc<RefCell<TreeNode>> = TreeNode::create_tree_8();
        println!("root: {:?}", root);
        let sum = Solution::sum_of_left_leaves(Some(root));
        println!("Left leaves sum: {sum}");
        assert_eq!(sum, 4)
    }
}
