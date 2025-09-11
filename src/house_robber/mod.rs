//! # House Robber I
//! This problem is listed on leetcode problems set.
//! ## Description:
//! You are a professional robber planning to rob houses along a street.
//! Each house has a certain amount of money stashed,
//! the only constraint stopping you from robbing each of them is that adjacent
//! houses have security systems connected and it will automatically contact the police
//! if two adjacent houses were broken into on the same night.
//! Given an integer array nums representing the amount of money of each house,
//! return the maximum amount of money you can rob tonight without alerting the police.
pub struct Solution;

impl Solution {
    pub fn rob(nums: Vec<i32>) -> i32 {
        if nums.len() == 1 {
            return nums[0];
        }
        let mut dp: Vec<i32> = vec![0i32; nums.len()];
        dp[0] = nums[0];
        dp[1] = nums[0].max(nums[1]);
        for i in 2..nums.len() {
            dp[i] = dp[i-1].max(dp[i-2] + nums[i]);
        }
        dp[nums.len() - 1]
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test1() {
        assert_eq!(Solution::rob(vec![1, 2, 3, 1]), 4);
    }

    #[test]
    fn test2() {
        assert_eq!(Solution::rob(vec![2, 7, 9, 3, 1]), 12);
    }

    #[test]
    fn test3() {
        assert_eq!(Solution::rob(vec![3, 2, 1]), 4);
    }

    #[test]
    fn test4() {
        assert_eq!(Solution::rob(vec![10, 1, 0, 11, 2, 13]), 34);
    }

    #[test]
    fn test5() {
        // Leetcode says each value in nums is between [0, 400]
        //assert_eq!(Solution::rob(vec![i32::MIN / 10, i32::MIN + 100, -1010010200, 0]), 0);
    }

    #[test]
    fn test6() {
        assert_eq!(Solution::rob(vec![40, 2, 4, 10]), 50);
    }
}