use std::cell::RefCell;
use std::rc::Rc;
use crate::data_structures::TreeNode;

/// This problem is listed on leetcode problems set
/// URL: https://leetcode.com/problems/path-sum-ii/description/
/// # Description:
/// Given the root of a binary tree and an integer targetSum,
/// return all root-to-leaf paths where the sum of the node values in the path
/// equals targetSum. Each path should be returned as a list of the node values, not node references.
/// A root-to-leaf path is a path starting from the root and ending at any leaf node.
/// A leaf is a node with no children.
pub struct Solution;

impl Solution {
    pub fn path_sum(root: Option<Rc<RefCell<TreeNode>>>, target_sum: i32) -> Vec<Vec<i32>> {
        match root {
            None => vec![],
            Some(root) => {
                let val = root.borrow().val;
                if root.borrow().left.is_none() && root.borrow().right.is_none() {
                    return if val == target_sum {
                        vec![vec![val]]
                    }else {
                        vec![]
                    }
                }
                let val = root.borrow().val;
                let mut path_l = Self::path_sum(root.borrow().left.clone(), target_sum - val);
                let mut path_r = Self::path_sum(root.borrow().right.clone(), target_sum - val);
                let mut path: Vec<Vec<i32>> = vec![];
                for mut l_p in path_l {
                    let mut v: Vec<i32> = vec![val];
                    v.append(&mut l_p);
                    path.push(v);
                }
                for mut r_p in path_r {
                    let mut v: Vec<i32> = vec![val];
                    v.append(&mut r_p);
                    path.push(v);
                }
                path
            }
        }
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
    ///                         /   \                  /   \
    ///                      (7)    (2)              (5)   (1)
    /// output: [[5,4,11,2],[5,8,4,5]]
    #[test]
    pub fn test1() {
        let root: Rc<RefCell<TreeNode>> = TreeNode::create_tree_12();
        assert_eq!(Solution::path_sum(Some(root), 22), [[5,4,11,2],[5,8,4,5]])
    }


    ///   # Example tree 1
    ///                                   (1)
    ///                                  /   \
    ///                                (2)   (3)
    /// output: []
    #[test]
    pub fn test2() {
        let root: Rc<RefCell<TreeNode>> = TreeNode::create_tree_1();
        assert_eq!(Solution::path_sum(Some(root), 5), Vec::<Vec<i32>>::new())
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
    /// Output: [[1, 15, -2, 30]]
    #[test]
    fn test3() {
        let root: Rc<RefCell<TreeNode>> = TreeNode::create_tree_8();
        assert_eq!(Solution::path_sum(Some(root), 44), [[1, 15, -2, 30]])
    }


    /// # Example tree 6:
    ///                         (2)
    ///                        /
    ///                     (-1)
    /// Output: []
    #[test]
    fn test4() {
        let root: Rc<RefCell<TreeNode>> = TreeNode::create_tree_6();
        assert_eq!(Solution::path_sum(Some(root), 3), Vec::<Vec<i32>>::new())
    }

    /// # Example tree 10
    ///
    ///                         (10)
    ///                        /   \
    ///                     (2)    (11)
    ///                    /
    ///                  (0)
    ///
    /// Output: [10, 2, 0]
    #[test]
    fn test5() {
        let root: Rc<RefCell<TreeNode>> = TreeNode::create_tree_10();
        assert_eq!(Solution::path_sum(Some(root), 12), [[10, 2, 0]])
    }

    /// # Example tree 11
    ///                             (-2)
    ///                                \
    ///                                (-3)
    ///
    /// Output: [-2, -3]
    #[test]
    fn test6(){
        let root: Rc<RefCell<TreeNode>> = TreeNode::create_tree_11();
        assert_eq!(Solution::path_sum(Some(root), -5), [[-2, -3]])
    }


    /// # Example tree 13
    ///                         (-1)
    ///                        /   \
    ///                     (2)    (0)
    ///                    /  \      \
    ///                 (0)   (0)    (2)
    ///
    /// Output: [[-1, 2, 0], [-1, 2, 0], [-1, 0, 2]]
    #[test]
    fn test7(){
        let root: Rc<RefCell<TreeNode>> = TreeNode::create_tree_13();
        assert_eq!(Solution::path_sum(Some(root), 1), [[-1, 2, 0], [-1, 2, 0], [-1, 0, 2]])
    }
}