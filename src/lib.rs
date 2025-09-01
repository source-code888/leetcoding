#![allow(unused_variables)]
#![allow(dead_code)]
#![allow(warnings)]

use std::cell::RefCell;
use std::collections::VecDeque;
use std::rc::Rc;
use crate::data_structures::TreeNode;

mod longest_palindromic_substring;
mod power_of_two;
mod zigzag;

mod atoi;
pub mod btlot;
mod course_schedule_3;
mod data_structures;
mod flood_fill;
mod max_depth_bt;
mod reordered_power_of_two;
mod reverse_integer;
mod two_sum;
mod btmpt;
mod palindrome_number;
mod soll;
mod best_time_to_buy_and_sell_i;
mod best_time_to_buy_and_sell_ii;
mod best_time_to_buy_and_sell_iii;
mod path_sum_i;
mod path_sum_ii;

fn dfs(root: Option<Rc<RefCell<TreeNode>>>) -> Option<i32> {
    if root.is_none() {
        return None;
    }
    let root = root.unwrap();
    let mut queue: VecDeque<Rc<RefCell<TreeNode>>> = VecDeque::new();
    let mut visited_nodes: Vec<Rc<RefCell<TreeNode>>> = vec![];
    queue.push_back(root.clone());
    visited_nodes.push(root);
    while !queue.is_empty() {
        let node = queue.pop_front().unwrap();
        let val = node.borrow().val;
        println!("Value: {val}");
        for neighbour in get_neighbours(node) {
            if !visited_nodes.contains(&neighbour) {
                queue.push_back(neighbour.clone());
                visited_nodes.push(neighbour);
            }
        }
    }
    Some(0i32)
}

fn get_neighbours(root: Rc<RefCell<TreeNode>>) -> Vec<Rc<RefCell<TreeNode>>> {
    let mut neighbours: Vec<Rc<RefCell<TreeNode>>> = vec![];
    if let Some(left) = root.borrow().left.clone() {
        neighbours.push(left);
    }
    if let Some(right) = root.borrow().right.clone() {
        neighbours.push(right);
    }
    neighbours
}

#[cfg(test)]
mod tests {
    use std::cell::RefCell;
    use std::rc::Rc;
    use crate::{btmpt, dfs};
    use crate::data_structures::TreeNode;

    #[test]
    fn test_one() {
        let root: Rc<RefCell<TreeNode>> = Rc::new(RefCell::new(TreeNode::new(-10)));
        root.borrow_mut().left = Some(Rc::new(RefCell::new(TreeNode::new(9))));
        let right: Rc<RefCell<TreeNode>> = Rc::new(RefCell::new(TreeNode::new(20)));
        right.borrow_mut().left = Some(Rc::new(RefCell::new(TreeNode::new(15))));
        right.borrow_mut().right = Some(Rc::new(RefCell::new(TreeNode::new(7))));
        root.borrow_mut().right = Some(right);

        println!("Root: {}", root.borrow());
        println!("Max path sum: {:?}", dfs(Some(root)))
    }
}