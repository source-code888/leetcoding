use rand::{random_bool, random_range};
/// # Data Structures
/// Here are the definitions for Data Structures that could be needed
/// These are definitions of Data Structures for LeetCode implementation solutions
use std::cell::RefCell;
use std::fmt::{Display, Formatter};
use std::rc::Rc;

#[derive(PartialEq, Eq, Debug)]
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
    pub fn create_tree_randomly(
        lower_bound: i32,
        upper_bound: i32,
        p: f64,
    ) -> Option<Rc<RefCell<TreeNode>>> {
        let val: i32 = random_range(lower_bound..=upper_bound);
        let root: Rc<RefCell<TreeNode>> = Rc::new(RefCell::new(TreeNode::new(val)));
        if random_bool(p) {
            root.borrow_mut().left = Self::create_tree_randomly(lower_bound, upper_bound, p);
        }
        if random_bool(p) {
            root.borrow_mut().right = Self::create_tree_randomly(lower_bound, upper_bound, p);
        }
        Some(root)
    }

    fn in_order_traversal(&self) -> String {
        let mut ret: String = String::new();
        if let Some(left) = &self.left {
            ret.push_str(&format!(" {} ", left.borrow().in_order_traversal()));
        }
        ret.push_str(&format!(" {} ", self.val));
        if let Some(right) = &self.right {
            ret.push_str(&format!(" {} ", right.borrow().in_order_traversal()));
        }
        ret
    }

    /// # Example test 2
    ///
    ///                             (-10)
    ///                            /     \
    ///                         (9 )    (20)
    ///                                /    \
    ///                             (15)    (7)
    pub fn create_tree_2() -> Rc<RefCell<TreeNode>> {
        let root: Rc<RefCell<TreeNode>> = Rc::new(RefCell::new(TreeNode::new(-10)));
        root.borrow_mut().left = Some(Rc::new(RefCell::new(TreeNode::new(9))));
        let right: Rc<RefCell<TreeNode>> = Rc::new(RefCell::new(TreeNode::new(20)));
        right.borrow_mut().left = Some(Rc::new(RefCell::new(TreeNode::new(15))));
        right.borrow_mut().right = Some(Rc::new(RefCell::new(TreeNode::new(7))));
        root.borrow_mut().right = Some(right);
        root
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
    pub fn create_tree_5() -> Rc<RefCell<TreeNode>> {
        let root: Rc<RefCell<TreeNode>> = Rc::new(RefCell::new(TreeNode::new(1)));
        let left: Rc<RefCell<TreeNode>> = Rc::new(RefCell::new(TreeNode::new(-2)));
        let left_l: Rc<RefCell<TreeNode>> = Rc::new(RefCell::new(TreeNode::new(1)));
        let left_l_l: Rc<RefCell<TreeNode>> = Rc::new(RefCell::new(TreeNode::new(-1)));
        left_l.borrow_mut().left = Some(left_l_l);
        left.borrow_mut().left = Some(left_l);
        left.borrow_mut().right = Some(Rc::new(RefCell::new(TreeNode::new(3))));
        root.borrow_mut().left = Some(left);
        let right: Rc<RefCell<TreeNode>> = Rc::new(RefCell::new(TreeNode::new(-3)));
        let right_l: Rc<RefCell<TreeNode>> = Rc::new(RefCell::new(TreeNode::new(-2)));
        right.borrow_mut().left = Some(right_l);
        root.borrow_mut().right = Some(right);
        root
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
    pub fn create_tree_8() -> Rc<RefCell<TreeNode>> {
        let root: Rc<RefCell<TreeNode>> = Rc::new(RefCell::new(TreeNode::new(1)));
        let left: Rc<RefCell<TreeNode>> = Rc::new(RefCell::new(TreeNode::new(10)));
        let left_l: Rc<RefCell<TreeNode>> = Rc::new(RefCell::new(TreeNode::new(-5)));
        left.borrow_mut().left = Some(left_l);
        let left_r: Rc<RefCell<TreeNode>> = Rc::new(RefCell::new(TreeNode::new(6)));
        let left_r_l: Rc<RefCell<TreeNode>> = Rc::new(RefCell::new(TreeNode::new(7)));
        let left_r_r: Rc<RefCell<TreeNode>> = Rc::new(RefCell::new(TreeNode::new(-20)));
        left_r.borrow_mut().left = Some(left_r_l);
        left_r.borrow_mut().right = Some(left_r_r);
        left.borrow_mut().right = Some(left_r);
        root.borrow_mut().left = Some(left);
        let right: Rc<RefCell<TreeNode>> = Rc::new(RefCell::new(TreeNode::new(15)));
        let right_l: Rc<RefCell<TreeNode>> = Rc::new(RefCell::new(TreeNode::new(2)));
        right.borrow_mut().left = Some(right_l);
        let right_r: Rc<RefCell<TreeNode>> = Rc::new(RefCell::new(TreeNode::new(-2)));
        let right_r_r: Rc<RefCell<TreeNode>> = Rc::new(RefCell::new(TreeNode::new(30)));
        right_r.borrow_mut().right = Some(right_r_r);
        right.borrow_mut().right = Some(right_r);
        root.borrow_mut().right = Some(right);
        root
    }
}


impl Display for TreeNode {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.in_order_traversal())
    }
}