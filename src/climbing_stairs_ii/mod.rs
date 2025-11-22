pub struct Solution;

impl Solution {
    pub fn climb_stairs(n: i32, costs: Vec<i32>) -> i32 {
        let mut dp: Vec<i32> = vec![i32::MAX; n as usize + 1];
        dp[0] = 0;
        for j in 1..n as usize + 1 {
            for i in 1..=3 {
                if i > j {
                    break;
                }
                let prev: usize = j - i;
                dp[j] = dp[j].min(dp[prev] + costs[j - 1] + ((j - prev) as i32).pow(2))
            }
        }
        dp[n as usize]
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test1() {
        assert_eq!(Solution::climb_stairs(4, vec![1, 2, 3, 4]), 13)
    }

    #[test]
    fn test2() {
        assert_eq!(Solution::climb_stairs(4, vec![5, 1, 6, 2]), 11)
    }

    #[test]
    fn test3() {
        assert_eq!(Solution::climb_stairs(3, vec![9, 8, 3]), 12)
    }
}
