use std::collections::HashMap;

/// # Climbing Stairs
/// This problem is listed on leetcode problems set
/// URL: https://leetcode.com/problems/climbing-stairs/description/
/// # Description
/// You are climbing a staircase. It takes n steps to reach the top.
/// Each time you can either climb 1 or 2 steps. In how many distinct ways can you climb to the top?
/// # Example 1:
/// n = 2
/// Explanation: There are two ways to climb to the top.
/// 1. 1 step + 1 step
/// 2. 2 steps
pub struct Solution;

impl Solution {
    fn climb_stairs(n: i32) -> i32 {
        if n < 1 || n > 45 {
            return 0;
        }
        let mut map: HashMap<i32, i32> = HashMap::new();
        map.insert(1, 1);
        map.insert(2, 2);
        Self::helper(n, &mut map)
    }

    fn helper(n: i32, map: &mut HashMap<i32, i32>) -> i32 {
        if map.contains_key(&n) {
            return *map.get(&n).unwrap();
        }
        let r: i32 = Solution::helper(n - 1, map) + Solution::helper(n - 2, map);
        map.insert(n, r);
        r
    }
}
#[cfg(test)]
mod tests {
    use super::Solution;
    #[test]
    fn test1() {
        assert_eq!(Solution::climb_stairs(2), 2);
    }

    #[test]
    fn test2() {
        assert_eq!(Solution::climb_stairs(3), 3);
    }

    #[test]
    fn test3() {
        assert_eq!(Solution::climb_stairs(4), 5);
    }

    #[test]
    fn test4() {
        assert_eq!(Solution::climb_stairs(5), 8);
    }

    #[test]
    fn test5() {
        assert_eq!(Solution::climb_stairs(6), 13);
    }

    #[test]
    fn test6() {
       assert_eq!(Solution::climb_stairs(45), 1_836_311_903);
    }
}
