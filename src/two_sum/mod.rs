/// # Two Sum
/// This problem is listed on leetcode problems set
/// URl: https://leetcode.com/problems/two-sum/
/// # Description
/// Given an array of integers nums and an integer target,
/// return indices of the two numbers such that they add up to target.
/// You may assume that each input would have exactly one solution, and you may not use the same element twice.
/// You can return the answer in any order.

pub struct Solution;

impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        if nums.len() < 2
            || nums.len() as i32 > 10i32.pow(4u32)
            || target < -10i32.pow(9u32)
            || target > 10i32.pow(9u32)
        {
            return vec![];
        }
        for i in 0..nums.len() {
            let mut start: usize = i;
            let mut end: usize = i + 1; // Since we can't use the same element to reach the target
            while end < nums.len() && nums[start] + nums[end] != target{
                end += 1
            }
            if end < nums.len() && nums[start] + nums[end] == target {
                return vec![start as i32, end as i32]
            }
        }
        vec![]
    }
}
