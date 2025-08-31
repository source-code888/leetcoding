/// This problem is listed on leetcode problems set
/// URL: https://leetcode.com/problems/best-time-to-buy-and-sell-stock-ii/description/
/// # Description:
/// You are given an integer array prices where prices[i] is the price of a given stock on the ith day.
/// On each day, you may decide to buy and/or sell the stock.
/// You can only hold at most one share of the stock at any time.
/// However, you can buy it then immediately sell it on the same day.
/// Find and return the maximum profit you can achieve.
pub struct Solution;

impl Solution {
    pub fn max_profit(prices: Vec<i32>) -> i32 {
        let mut profit_after_buying: Vec<i32> = vec![i32::MIN; prices.len() / 2 + 1];
        let mut profit_after_selling: Vec<i32> = vec![0; prices.len() / 2 + 1];
        for i in 0..prices.len() {
            let curr_price = prices[i];
            for j in (1..prices.len() / 2 + 1).rev() {
                profit_after_buying[j] = profit_after_buying[j].max(profit_after_selling[j - 1] - curr_price);
                profit_after_selling[j] = profit_after_selling[j].max(profit_after_buying[j] + curr_price);
            }
        }
        profit_after_selling[prices.len() / 2 ]
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn case1() {
        let prices: Vec<i32> = vec![7,1,5,3,6,4];
        assert_eq!(Solution::max_profit(prices), 7);
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