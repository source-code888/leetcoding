//! # Median of Two Sorted Arrays
//! ## DESCRIPTION:
//! Given two sorted arrays nums1 and nums2 of size m and n respectively,
//! return the median of the two sorted arrays.
//! The overall run time complexity should be O(log (m+n)).
pub struct Solution;

impl Solution {
    pub fn find_median_sorted_arrays(nums1: Vec<i32>, nums2: Vec<i32>) -> f64 {
        let mut nums: Vec<i32> = nums1;
        nums.append(&mut nums2.into_iter().collect::<Vec<i32>>());
        nums.sort();
        let mid = nums.len() / 2;
        if nums.len() % 2 == 0 {
            (nums[mid-1] + nums[mid]) as f64 / 2f64
        }else {
            nums[mid] as f64
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(Solution::find_median_sorted_arrays(vec![1, 3], vec![2]), 2f64)
    }

    #[test]
    fn test2() {
        assert_eq!(Solution::find_median_sorted_arrays(vec![1, 2], vec![3, 4]), 2.5)
    }
}