//! # 3 SUM
//! ## Problem description:
//! Given an integer array nums, return all the triplets \[nums\[i], nums\[j], nums\[k]]
//! such that i != j, i != k, and j != k, and nums\[i] + nums\[j] + nums\[k] == 0.
//! Notice that the solution set must not contain duplicate triplets.
//! ## Approach
//! Two pointers
//! - We can sort the input array and set two pointers, one to the end and one
//! to **index** + 1.
//! - If we iterate through **nums**, then, we can select **nums\[index]** and set a
//! **target**, this target is equal to **-nums\[index]**.
//! - There are still two numbers missing to complete the triplet, so we set one variable
//! **start** to **index+1** and **end** to **nums.len()-1**. This way we search two numbers such
//! that the sum of both is equal to **target**.
pub struct Solution;

impl Solution {
    pub fn three_sum(nums: Vec<i32>) -> Vec<Vec<i32>> {
        if nums.len() < 3 {
            return vec![];
        }
        let mut nums = nums;
        nums.sort();
        let mut ans: Vec<Vec<i32>> = vec![];
        for i in 0..nums.len() - 2 {
            let x: i32 = nums[i];
            let mut left: usize = i + 1;
            let mut right: usize = nums.len() - 1;
            let target = -x;
            while left < right {
                let sum = nums[left] + nums[right];
                if sum > target {
                    right -= 1;
                } else if sum == target {
                    let els: Vec<i32> = vec![x, nums[left], nums[right]];
                    if !ans.contains(&els) {
                        ans.push(els);
                    }
                    right -= 1;
                    left += 1
                } else {
                    left += 1
                }
            }
        }
        ans
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn example1() {
        assert_eq!(
            Solution::three_sum(vec![-1, 0, 1, 2, -1, -4]),
            [[-1, -1, 2], [-1, 0, 1]]
        );
    }

    #[test]
    fn example2() {
        let r = Solution::three_sum(vec![0, 1, 1]);
        assert_eq!(r, Vec::<Vec<i32>>::new());
    }

    #[test]
    fn example3() {
        assert_eq!(
            Solution::three_sum(vec![-100, -70, -60, 110, 120, 130, 160]),
            [[-100, -60, 160], [-70, -60, 130]]
        );
    }

    #[test]
    fn example4() {
        // -910 -90 0 9 81 90 1000
        assert_eq!(
            Solution::three_sum(vec![-90, 9, 81, 90, 0, 1000, -910]),
            [[-910, -90, 1000],[-90, 0, 90], [-90, 9, 81]]
        );
    }

    #[test]
    fn example5() {
        assert_eq!(Solution::three_sum(vec![0, 0, 0]), [[0, 0, 0]]);
    }
}
