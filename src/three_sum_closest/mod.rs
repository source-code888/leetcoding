//! # 3 SUM CLOSEST
//! ## Problem description:
//! Given an integer array nums of length n and an integer target,
//! find three integers in nums such that the sum is closest to **target**.
//! Return the sum of the three integers.
//! You may assume that each input would have exactly one solution.
//! ## Approach
//! Two pointers
//! - Similar to 3 SUM
//! - After sorting **nums**, we can take **nums\[i]**, **nums\[left]** and **nums\[right]**.
//! - If we sum these three numbers, we can set a variable **min** and keep the **minimum** difference
//! between the **sum** and **target**.
//! - If the difference is 0, we return **nums\[i] + nums\[left] + nums\[right]**.
//! - If not, we decrement **right** if **sum** is greater than **target**.
//! - Otherwise, we increment **left**.
pub struct Solution;
impl Solution {
    pub fn three_sum_closest(nums: Vec<i32>, target: i32) -> i32 {
        if nums.len() < 3 {
            return 0;
        }
        let mut result = nums[0] + nums[1] + nums[2];
        if nums.len() == 3 {
            return result;
        }
        let mut nums: Vec<i32> = nums;
        let mut min = (target - result).abs();
        nums.sort();
        for i in 0..nums.len() - 2 {
            let mut l: usize = i + 1;
            let mut r: usize = nums.len() - 1;
            while l < r {
                let sum = nums[l] + nums[r] + nums[i];
                let diff = (target - sum).abs();
                if sum == target {
                    return sum;
                } else if sum > target {
                    r -= 1
                } else {
                    l += 1
                }
                if diff < min {
                    min = diff;
                    result = sum
                }
            }
        }
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(Solution::three_sum_closest(vec![-1, 2, 1, -4], 1), 2)
    }

    #[test]
    fn test2() {
        assert_eq!(Solution::three_sum_closest(vec![0, 0, 0], 1), 0)
    }

    #[test]
    fn test3() {
        assert_eq!(
            Solution::three_sum_closest(vec![21, 105, 91, -10, 9, 5, 15, -5, -1, 1], 100),
            100
        )
    }

    #[test]
    fn test4() {
        assert_eq!(
            Solution::three_sum_closest(vec![10, 10, 10, -9, -50, 100], 90),
            101
        )
    }

    #[test]
    fn test5() {
        assert_eq!(
            Solution::three_sum_closest(
                vec![
                    93, 91, 42, 103, -59, -90, 58, 72, -50, 4, 41, -43, -10, 115, -49
                ],
                14
            ),
            15
        )
    }

    #[test]
    fn test6() {
        assert_eq!(
            Solution::three_sum_closest(
                vec![
                    -63, -94, -92, 69, -58, 68, 43, -85, 108, -50, -114, 19, 53, -98, 31
                ],
                58
            ),
            58
        )
    }

    #[test]
    fn test7() {
        assert_eq!(
            Solution::three_sum_closest(vec![1000, -1000, 2000, -2000, 500], 300),
            500
        )
    }

    #[test]
    fn test8() {
        assert_eq!(
            Solution::three_sum_closest(vec![5, 20, 40, 1, 2, 3], 50),
            48
        )
    }

    #[test]
    fn test9() {
        assert_eq!(Solution::three_sum_closest(vec![-10, 0, 0], -10), -10)
    }

    #[test]
    fn test10() {
        assert_eq!(
            Solution::three_sum_closest(
                vec![-15, 16, 45, 34, 24, 56, -14, -5, 6, 0, -1, 2, 36],
                45
            ),
            45
        )
    }
}
