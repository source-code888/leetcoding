#![allow(unused_variables)]
#![allow(dead_code)]
#![allow(warnings)]

use std::cell::RefCell;
use std::fmt::format;
use std::rc::Rc;
use crate::data_structures::TreeNode;

mod longest_palindromic_substring;
mod zigzag;
mod power_of_two;

mod reverse_integer;
mod reordered_power_of_two;
mod atoi;
pub mod btlot;
mod flood_fill;
mod data_structures;
mod max_depth_bt;

///
/// let image: Vec<Vec<i32>> = vec![vec![1,1,1],vec![1,1,0],vec![1,0,1]];
//     let sr = 1;
//     let sc = 1;
//     let color = 2;
//
//     println!("Image: {image:?}");
//
//     println!("Flood: {:?}", flood_fill::Solution::flood_fill(image, sr, sc, color))

fn main() {
    let root: Option<Rc<RefCell<TreeNode>>> = TreeNode::create_tree_randomly(-100, 100, 0.45f64);
    println!("{}", root.as_ref().unwrap().borrow());
    println!("Max depth: {}", max_depth_bt::Solution::max_depth(root));
}
