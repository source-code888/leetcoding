#![allow(unused_variables)]
#![allow(dead_code)]
#![allow(warnings)]

use crate::data_structures::TreeNode;
use std::cell::RefCell;
use std::fmt::format;
use std::rc::Rc;

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
    let courses: Vec<Vec<i32>> = vec![vec![7,16],vec![2,3],vec![3,12],vec![3,14],vec![10,19],vec![10,16],vec![6,8],vec![6,11],vec![3,13],vec![6,16]];
    println!("Courses: {courses:?}");
    println!(
        "Max courses: {}",
        course_schedule_3::Solution::schedule_course(courses)
    )
}
