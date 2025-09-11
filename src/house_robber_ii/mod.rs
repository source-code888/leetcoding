//! # House Robber II
//! This problem is listed on leetcode problems set
//! ## Description:
//! You are a professional robber planning to rob houses along a street.
//! Each house has a certain amount of money stashed.
//! All houses at this place are arranged in a circle.
//! That means the first house is the neighbor of the last one.
//! Meanwhile, adjacent houses have a security system connected,
//! and it will automatically contact the police if two adjacent
//! houses were broken into on the same night.
//! Given an integer array nums representing the amount of money of each house,
//! return the maximum amount of money you can rob tonight without alerting the police.
pub struct Solution;

impl Solution {

    pub fn rob(nums: Vec<i32>) -> i32 {
        if nums.len() == 1 {
            return nums[0];
        }
        Self::helper(&nums[0..nums.len() - 1]).max(Self::helper(&nums[1..nums.len()]))
    }

    fn helper(nums: &[i32]) -> i32 {
        let mut dp: Vec<i32> = vec![0; nums.len()];
        dp[0] = nums[0];
        dp[1] = nums[0].max(nums[1]);
        for i in 2..nums.len() {
            dp[i] = dp[i-1].max(dp[i-2] + nums[i]);
        }
        dp[nums.len()-1]
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(Solution::rob(vec![2, 3, 2]), 3);
    }

    #[test]
    fn test2() {
        assert_eq!(Solution::rob(vec![1, 2, 3, 1]), 4);
    }

    #[test]
    fn test3() {
        assert_eq!(Solution::rob(vec![1, 2, 3]), 3);
    }

    #[test]
    fn test4() {
        assert_eq!(Solution::rob(vec![10, 11, 2, 4, 30]), 41);
    }
}