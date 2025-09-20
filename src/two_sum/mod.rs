//! # Two Sum
//! This problem is listed on leetcode problems set
//! URl: https://leetcode.com/problems/two-sum/
//! # Description
//! Given an array of integers nums and an integer target,
//! return indices of the two numbers such that they add up to target.
//! You may assume that each input would have exactly one solution, and you may not use the same element twice.
//! You can return the answer in any order.

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
        for i in 0..nums.len() - 1 {
            let mut end: usize = nums.len() - 1;
            while end > i + 1 && nums[i] + nums[end] != target{
                end -= 1
            }
            if nums[i] + nums[end] == target {
                return vec![i as i32, end as i32]
            }
        }
        vec![]
    }

    pub fn two_sum_p(nums: Vec<i32>, target: i32) -> Vec<i32> {
        if nums.len() < 2 {
            return vec![];
        }
        let mut nums: Vec<i32> = nums;
        nums.sort();
        for i in 0..nums.len() - 1{
            let target = target - nums[i];
            if let Some(j) = Self::helper(&nums[i + 1..], target) {
                return vec![i as i32, j]
            }
        }
        vec![]
    }

    fn helper(nums: &[i32], target: i32) -> Option<i32> {
        let mut l = 0usize;
        let mut r:usize = nums.len() - 1;
        while l <= r {
            let mid: usize = l + (r - l) / 2;
            if nums[mid] == target {
                return Some(mid as i32)
            } else if nums[mid] > target {
                r = mid - 1
            }else {
                l = mid + 1
            }
        }
        None
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test1 (){
        assert_eq!(Solution::two_sum_p(vec![2, 7, 11, 15], 9), vec![0, 1])
    }

    #[test]
    fn test2 () {
        assert_eq!(Solution::two_sum_p(vec![3, 2, 4], 6), vec![1, 2])
    }

}
