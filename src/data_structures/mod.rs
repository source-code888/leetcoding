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
        let mut ret: String = String::from("[");
        if let Some(left) = &self.left {
            ret.push_str(&format!("l: {} ", left.borrow().in_order_traversal()));
        }
        ret.push_str(&format!("root: {} ", self.val));
        if let Some(right) = &self.right {
            ret.push_str(&format!("r: {} ", right.borrow().in_order_traversal()));
        }
        ret.push(']');
        ret
    }
}


impl Display for TreeNode {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.in_order_traversal())
    }
}