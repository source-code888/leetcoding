/// # Best Time to Buy and Sell Stock III
/// This problem is listed on leetcode problems set
/// URL: https://leetcode.com/problems/best-time-to-buy-and-sell-stock-iii/
/// # Description:
/// You are given an array prices where prices[i] is the price of a given stock on the ith day.
/// Find the maximum profit you can achieve. You may complete at most two transactions.
/// Note: You may not engage in multiple transactions simultaneously
/// (i.e., you must sell the stock before you buy again).
pub struct Solution;

impl Solution {
    pub fn max_profit(prices: Vec<i32>) -> i32 {
        let mut profit: (i32, i32) = (0, 0);
        let mut profit_aux: (i32, i32) = (i32::MIN, i32::MIN);
        for i in 0..prices.len() {
            profit_aux.1 = profit_aux.1.max(profit.0 - prices[i]);
            profit.1 = profit.1.max(profit_aux.1 + prices[i]);
            profit_aux.0 = profit_aux.0.max( - prices[i]);
            profit.0 = profit.0.max(profit_aux.0 + prices[i]);
        }
        profit.1
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn case1() {
        let prices: Vec<i32> = vec![3,3,5,0,0,3,1,4];
        assert_eq!(Solution::max_profit(prices), 6);
    }

    #[test]
    fn case2() {
        let prices: Vec<i32> = vec![1,2,3,4,5];
        assert_eq!(Solution::max_profit(prices), 4);
    }

    #[test]
    fn case3() {
        let prices: Vec<i32> = vec![7,6,4,3,1];
        assert_eq!(Solution::max_profit(prices), 0);
    }
}