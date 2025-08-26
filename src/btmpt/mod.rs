use crate::data_structures::TreeNode;
use std::cell::RefCell;
use std::rc::Rc;
/// # Binary Tree Maximum Path Sum
/// This problem is listed on LeetCode problems set
/// URL: https://leetcode.com/problems/binary-tree-maximum-path-sum/description/
/// # Description
/// A path in a binary tree is a sequence of nodes where each pair of adjacent nodes in the sequence has an edge
/// connecting them. A node can only appear in the sequence at most once.
/// Note that the path does not need to pass through the root.
///
/// The path sum of a path is the sum of the node's values in the path.
///
/// Given the root of a binary tree, return the maximum path sum of any non-empty path.
pub struct Solution;

impl Solution {

    /// This solution is not mine, I gave up.
    /// The URL where I found an explanation of this approach is the next:
    /// https://interviewnoodle.com/amazon-sde-interview-experience-on-campus-e8444ee791b
    /// I was implementing a DFS algorithm but I did not figure out how to keep track to the max path sum.
    pub fn max_path_sum(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        let (root_max_depth, root_overall_depth) = Self::dfs_max_path(root);
        root_max_depth.max(root_overall_depth)
    }

    fn dfs_max_path(root: Option<Rc<RefCell<TreeNode>>>) -> (i32, i32) {
        if root.is_none() {
            return (0, i32::MIN);
        }
        let root = root.unwrap();
        let val = root.borrow().val;
        if root.borrow().left.is_none() && root.borrow().right.is_none() {
            return (val, val);
        }
        let (left_root_depth, left_overall_depth) = Self::dfs_max_path(root.borrow().left.clone());
        let (right_root_depth, right_overall_depth) =
            Self::dfs_max_path(root.borrow().right.clone());
        let best_path = (left_root_depth + val).max(right_root_depth + val);
        let current_depth = left_root_depth + val + right_root_depth;
        let depth_with_current_node = val.max(best_path).max(current_depth);
        let overall_max = left_overall_depth.max(right_overall_depth).max(depth_with_current_node);
        let best_current_root_path = val.max(best_path);
        (best_current_root_path, overall_max)
    }
}

#[cfg(test)]
pub mod tests {
    use crate::btmpt;
    use crate::data_structures::TreeNode;
    use std::cell::RefCell;
    use std::rc::Rc;
    use crate::btmpt::Solution;

    ///   # Example Test 1
    ///                                   (1)
    ///                                  /   \
    ///                                (2)   (3)
    ///
    /// Max path sum: 6
    #[test]
    fn case_1() {
        let root: Rc<RefCell<TreeNode>> = Rc::new(RefCell::new(TreeNode::new(1)));
        root.borrow_mut().left = Some(Rc::new(RefCell::new(TreeNode::new(2))));
        root.borrow_mut().right = Some(Rc::new(RefCell::new(TreeNode::new(3))));

        println!("Root: {}", root.borrow());
        let max_path_sum = btmpt::Solution::max_path_sum(Some(root));
        println!("Max path sum: {max_path_sum}");

        assert_eq!(max_path_sum, 6);
    }

    /// # Example test 2
    ///
    ///                             (-10)
    ///                            /     \
    ///                         (9 )    (20)
    ///                                /    \
    ///                             (15)    (7)
    ///
    /// Max path sum: 42
    #[test]
    fn case_2() {
        let root: Rc<RefCell<TreeNode>> = TreeNode::create_tree_2();
        println!("Root: {}", root.borrow());
        let max_path_sum = btmpt::Solution::max_path_sum(Some(root));
        println!("Max path sum: {max_path_sum}");

        assert_eq!(max_path_sum, 42);
    }

    /// # Example test 3
    ///                     (-3)
    ///
    /// Max path sum: -3
    #[test]
    fn case_3() {
        let root: Rc<RefCell<TreeNode>> = Rc::new(RefCell::new(TreeNode::new(-3)));
        println!("Root: {}", root.borrow());
        let max_path_sum = btmpt::Solution::max_path_sum(Some(root));
        println!("Max path sum: {max_path_sum}");

        assert_eq!(max_path_sum, -3)
    }

    /// # Example test 4
    ///                         (-2)
    ///                        /
    ///                     (-1)
    ///
    /// Max path sum: -1
    #[test]
    fn case_4() {
        let root: Rc<RefCell<TreeNode>> = Rc::new(RefCell::new(TreeNode::new(-2)));
        root.borrow_mut().left = Some(Rc::new(RefCell::new(TreeNode::new(-1))));
        println!("Root: {}", root.borrow());
        let max_path_sum = btmpt::Solution::max_path_sum(Some(root));
        println!("Max path sum: {max_path_sum}");

        assert_eq!(max_path_sum, -1)
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
    ///
    /// Max path sum: 3
    #[test]
    fn case_5() {
        let root: Rc<RefCell<TreeNode>> = TreeNode::create_tree_5();
        println!("Root: {}", root.borrow());
        let max_path_sum = btmpt::Solution::max_path_sum(Some(root));
        println!("Max path sum: {max_path_sum}");
        assert_eq!(max_path_sum, 3)
    }

    #[test]
    fn case_6() {
        let root: Rc<RefCell<TreeNode>> = Rc::new(RefCell::new(TreeNode::new(2)));
        root.borrow_mut().left = Some(Rc::new(RefCell::new(TreeNode::new(-1))));

        println!("Root: {}", root.borrow());
        let max_path_sum = btmpt::Solution::max_path_sum(Some(root));
        println!("Max path sum: {max_path_sum}");
        assert_eq!(max_path_sum, 2)
    }

    #[test]
    fn case_7() {
        let root: Rc<RefCell<TreeNode>> = Rc::new(RefCell::new(TreeNode::new(8)));
        root.borrow_mut().left = Some(Rc::new(RefCell::new(TreeNode::new(3))));

        let right = Rc::new(RefCell::new(TreeNode::new(12)));
        let right_l: Rc<RefCell<TreeNode>> = Rc::new(RefCell::new(TreeNode::new(10)));
        let right_r: Rc<RefCell<TreeNode>> = Rc::new(RefCell::new(TreeNode::new(4)));

        right.borrow_mut().left = Some(right_l);
        right.borrow_mut().right = Some(right_r);
        root.borrow_mut().right = Some(right);
        println!("Root: {}", root.borrow());
        let max_path_sum = btmpt::Solution::max_path_sum(Some(root));
        println!("Max path sum: {max_path_sum}");
        assert_eq!(max_path_sum, 33)
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
    ///
    ///  Max depth: 67
    #[test]
    fn case_8(){
        let root: Rc<RefCell<TreeNode>> = TreeNode::create_tree_8();
        println!("Root: {}", root.borrow());
        let max_path_sum = btmpt::Solution::max_path_sum(Some(root));
        println!("Max path sum: {max_path_sum}");
        assert_eq!(max_path_sum, 67)
    }
}
