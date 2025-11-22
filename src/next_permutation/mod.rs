//! # Next Permutation
//! ## Problem description
//! A permutation of an array of integers is an arrangement of its members into a sequence or linear order.
//! For example, for arr = \[1,2,3], the following are all the permutations of arr: \[1,2,3], \[1,3,2], \[2, 1, 3], \[2, 3, 1], \[3,1,2], \[3,2,1].
//! The next permutation of an array of integers is the next lexicographically greater permutation of its integer.
//! More formally, if all the permutations of the array are sorted in one container according to their lexicographical order,
//! then the next permutation of that array is the permutation that follows it in the sorted container.
//! If such arrangement is not possible, the array must be rearranged as the lowest possible order (i.e., sorted in ascending order).
//! For example, the next permutation of arr = \[1,2,3] is \[1,3,2].
//! Similarly, the next permutation of arr = \[2,3,1] is \[3,1,2].
//! While the next permutation of arr = \[3,2,1] is \[1,2,3] because \[3,2,1] does not have a lexicographical larger rearrangement.
//! Given an array of integers nums, find the next permutation of nums.
//! The replacement must be in place and use only constant extra memory.
pub struct Solution;
impl Solution {
    pub fn next_permutation(nums: &mut Vec<i32>) {
        match Self::find_largest(nums) {
            None => nums.sort(),
            Some(k) => {
                let mut max_index: usize = k + 1;
                for j in k + 1..nums.len() {
                    if nums[k] < nums[j] {
                        max_index = j
                    }
                }
                let temp = nums[k];
                nums[k] = nums[max_index];
                nums[max_index] = temp;
                nums[k + 1..].reverse()
            }
        }
    }
    /// Check whether the input is at its final stage or not
    fn find_largest(nums: &Vec<i32>) -> Option<usize> {
        for i in (1..nums.len()).rev() {
            if nums[i] > nums[i - 1] {
                return Some(i - 1);
            }
        }
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test1() {
        let mut perm: Vec<i32> = vec![1, 2, 3];
        Solution::next_permutation(&mut perm);
        assert_eq!(perm, vec![1, 3, 2]);
    }

    #[test]
    fn test2() {
        let mut perm: Vec<i32> = vec![1, 1, 5];
        Solution::next_permutation(&mut perm);
        assert_eq!(perm, [1, 5, 1])
    }

    #[test]
    fn test3() {
        let mut perm: Vec<i32> = vec![3, 2, 1];
        Solution::next_permutation(&mut perm);
        assert_eq!(perm, [1, 2, 3])
    }

    #[test]
    fn test4() {
        let mut perm: Vec<i32> = vec![5, 4, 7, 5, 3, 2];
        Solution::next_permutation(&mut perm);
        assert_eq!(perm, [5, 5, 2, 3, 4, 7])
    }
}
