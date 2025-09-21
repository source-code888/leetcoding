//! # Two Sum II - Input array is sorted
//! ## DESCRIPTION:
//! Given a 1-indexed array of integer numbers that is already sorted in non-decreasing order,
//! find two numbers such that they add up to a specific target number.
//! Let these two numbers be numbers\[index1] and numbers\[index2] where 1 <= index1 < index2 <= numbers.length.
//! Return the indices of the two numbers, index1 and index2, added by one as an integer array \[index1, index2] of length 2.
//! The tests are generated such that there is exactly one solution. You may not use the same element twice.
//! Your solution must use only constant extra space.
//! ## Approach:
//! Two Pointers
pub struct Solution;

impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        if nums.len() < 2 {
            return vec![];
        }
        let mut l = 0usize;
        let mut r = nums.len() - 1;
        while l < r {
            let sum = nums[l] + nums[r];
            if sum == target {
                return vec![l as i32 + 1, r as i32 + 1];
            } else if sum > target {
                r -= 1
            } else {
                l += 1
            }
        }
        vec![]
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    pub fn test1() {
        assert_eq!(Solution::two_sum(vec![2, 7, 11, 15], 9), vec![1, 2]);
    }

    #[test]
    pub fn test2() {
        assert_eq!(Solution::two_sum(vec![2, 3, 4], 6), vec![1, 3]);
    }

    #[test]
    pub fn test3() {
        assert_eq!(Solution::two_sum(vec![-1, 0], -1), vec![1, 2]);
    }

    #[test]
    pub fn test4() {
        assert_eq!(Solution::two_sum(vec![3, 2, 4], -1), vec![]);
    }

}
