/// This problem is listed on leetcode problems set
/// URL: 
use std::cell::RefCell;
use std::collections::VecDeque;
use std::rc::Rc;


use crate::data_structures::TreeNode;

pub struct Solution;


impl Solution {
    pub fn level_order(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<Vec<i32>> {
        if root.is_none() {
            return vec![];
        }
        let mut result: Vec<Vec<i32>> = vec![];
        let root: Option<Rc<RefCell<TreeNode>>> = root;
        let mut queue: VecDeque<Rc<RefCell<TreeNode>>> = VecDeque::new();
        queue.push_back(root.unwrap());
        while  queue.len() > 0 {
            let leve_size = queue.len();
            let mut current_level: Vec<i32> = vec![];
            for _ in 0..leve_size {
                if let Some(node) = queue.pop_front() {
                    current_level.push(node.borrow().val);
                    if let Some(left) = node.borrow().left.clone() {
                        queue.push_back(left)
                    }
                    if let Some(right) = node.borrow().right.clone() {
                        queue.push_back(right)
                    }
                }
            }
            result.push(current_level);
        }
        result
    }
}
