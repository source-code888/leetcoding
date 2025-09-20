//! # 4 Sum
//! ## Description:
//! Given an array nums of n integers, return an array of all the unique quadruplets
//! \[nums\[a], nums\[b], nums\[c], nums\[d]] such that:
//! 0 <= a, b, c, d < n
//! a, b, c, and d are distinct.
//! nums\[a] + nums\[b] + nums\[c] + nums\[d] == target
//! You may return the answer in any order.
//! ## Approach:
//!
pub struct Solution;

impl Solution {
    pub fn four_sum(nums: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
        todo!("I don't want to use O(n^3) solution to solve this.")
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test1() {
        assert_eq!(
            Solution::four_sum(vec![1, 0, -1, 0, -2, 2], 0),
            [[-2, -1, 1, 2], [-2, 0, 0, 2], [-1, 0, 0, 1]]
        )
    }

    #[test]
    fn test2() {
        assert_eq!(Solution::four_sum(vec![2, 2, 2, 2, 2], 8), [[2, 2, 2, 2]])
    }

    #[test]
    fn test3() {
        assert_eq!(
            Solution::four_sum(vec![2, 2, 2, 2, 2, -5, 7, 10, -3, 4, 0, 8], 9),
            [
                [-5, -3, 7, 10],
                [-5, 0, 4, 10],
                [-5, 2, 2, 10],
                [-5, 2, 4, 8],
                [-3, 0, 2, 10],
                [-3, 0, 4, 8],
                [-3, 2, 2, 8]
            ]
        )
    }

    #[test]
    fn test4() {
        assert_eq!(
            Solution::four_sum(vec![-3, -7, -8, -11, 1, 10, 11, -1], 0),
            [[-11, -1, 1, 11], [-8, -3, 1, 10], [-7, -3, -1, 11]]
        )
    }

    #[test]
    fn test5() {
        assert_eq!(
            Solution::four_sum(vec![-2, -2, -4, 8, -16, 0, -4, 0, 0, -8], -8),
            [
                [-16, 0, 0, 8],
                [-8, -4, -4, 8],
                [-8, 0, 0, 0],
                [-4, -4, 0, 0],
                [-4, -2, -2, 0]
            ]
        )
    }

    #[test]
    fn test6() {
        assert_eq!(
            Solution::four_sum(vec![0, 0, 0, 0, 1, 1, 1, 1], 2),
            [[0, 0, 1, 1]]
        )
    }

    #[test]
    fn test7() {
        assert_eq!(
            Solution::four_sum(vec![1, 1, 1, 1], 1),
            Vec::<Vec<i32>>::new()
        )
    }

    #[test]
    fn test8() {
        assert_eq!(
            Solution::four_sum(vec![-7, -6, -4, -3, -2, -1, 0, 1, 2, 4, 5, 6], -10),
            [
                [-7, -6, -3, 6],
                [-7, -6, -2, 5],
                [-7, -6, -1, 4],
                [-7, -6, 1, 2],
                [-7, -4, -3, 4],
                [-7, -4, -1, 2],
                [-7, -4, 0, 1],
                [-7, -3, -2, 2],
                [-7, -3, -1, 1],
                [-7, -2, -1, 0],
                [-6, -4, -2, 2],
                [-6, -4, -1, 1],
                [-6, -3, -2, 1],
                [-6, -3, -1, 0],
                [-4, -3, -2, -1]
            ]
        )
    }

    #[test]
    fn test9() {
        assert_eq!(
            Solution::four_sum(vec![-3, -1, 0, 2, 4, 5], 0),
            [[-3, -1, 0, 4]]
        )
    }

    #[test]
    fn test10() {
        assert_eq!(
            Solution::four_sum(vec![-3, -1, 0, 2, 4, 5], 1),
            [[-3, -1, 0, 5]]
        )
    }

    #[test]
    fn test11() {
        assert_eq!(
            Solution::four_sum(vec![-5, -4, -3, -2, -1, 0, 0, 1, 2, 3, 4, 5], 0),
            [
                [-5, -4, 4, 5],
                [-5, -3, 3, 5],
                [-5, -2, 2, 5],
                [-5, -2, 3, 4],
                [-5, -1, 1, 5],
                [-5, -1, 2, 4],
                [-5, 0, 0, 5],
                [-5, 0, 1, 4],
                [-5, 0, 2, 3],
                [-4, -3, 2, 5],
                [-4, -3, 3, 4],
                [-4, -2, 1, 5],
                [-4, -2, 2, 4],
                [-4, -1, 0, 5],
                [-4, -1, 1, 4],
                [-4, -1, 2, 3],
                [-4, 0, 0, 4],
                [-4, 0, 1, 3],
                [-3, -2, 0, 5],
                [-3, -2, 1, 4],
                [-3, -2, 2, 3],
                [-3, -1, 0, 4],
                [-3, -1, 1, 3],
                [-3, 0, 0, 3],
                [-3, 0, 1, 2],
                [-2, -1, 0, 3],
                [-2, -1, 1, 2],
                [-2, 0, 0, 2],
                [-1, 0, 0, 1]
            ]
        )
    }

    #[test]
    fn test12() {
        assert_eq!(
            Solution::four_sum(
                vec![1_000_000_000, 1000000000, 1000000000, 1000000000],
                -294967296
            ),
            Vec::<Vec<i32>>::new()
        )
    }

    #[test]
    fn test13() {
        assert_eq!(
            Solution::four_sum(
                vec![0, 0, 0, 1000000000, 1000000000, 1000000000, 1000000000],
                1_000_000_000
            ),
            [[0, 0, 0, 1_000_000_000]]
        )
    }
}
