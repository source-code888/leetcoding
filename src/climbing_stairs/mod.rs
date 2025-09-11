//! # Climbing Stairs
//! This problem is listed on leetcode problems set
//! URL: https://leetcode.com/problems/climbing-stairs/description/
//! ## Description
//! You are climbing a staircase. It takes n steps to reach the top.
//! Each time you can either climb 1 or 2 steps. In how many distinct ways can you climb to the top?
//! ## Approach
//! DP
//! This problem is similar to the classical fibonacci problem.
//! - We can climb 1 or 2 steps
//! - If *n* is equal to **1**, then we can just climb **1 step**.
//! - If **n** is equal to **2**, then we can climbi by **1 + 1 step** and by **2 steps**.
//! - If **n** is greater than **2**, we can climb **n - 2 steps** and **n - 1 steps**.
//! - This approach uses recursive functions, so with high inputs it will be slow. To make it
//! faster we can use **memoization**, so we do not calculate **climbing_stairs(n)** more than once.
//! - It can be solved with for loop, but I rather solve this by recursive functions.
use std::collections::HashMap;
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
