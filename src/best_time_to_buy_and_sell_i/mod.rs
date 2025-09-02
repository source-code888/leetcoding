/// # Best Time to Buy and Sell Stock I
/// This problem is listed on leetcode problems set.
/// URL: https://leetcode.com/problems/best-time-to-buy-and-sell-stock/description/
/// # Description:
/// You are given an array prices where prices[i] is the price of a given stock on the ith day.
/// You want to maximize your profit by choosing a single day to buy one stock and choosing a different
/// day in the future to sell that stock.
/// Return the maximum profit you can achieve from this transaction. If you cannot achieve any profit, return 0.
pub struct Solution;

impl Solution {
    pub fn max_profit(prices: Vec<i32>) -> i32 {
        let mut profit = 0i32;
        let mut min = prices[0];
        for i in 1..prices.len(){
            let curr_price = prices[i];
            if curr_price < min {
                min = curr_price
            } else {
                profit = profit.max(curr_price - min);
            }
        }
        profit
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    pub fn case_1() {
        let prices: Vec<i32> = vec![7,1,5,3,6,4];
        assert_eq!(Solution::max_profit(prices), 5);
    }


    #[test]
    pub fn case_2() {
        let prices: Vec<i32> = vec![7,6,4,3,1];
        assert_eq!(Solution::max_profit(prices), 0);
    }

    #[test]
    pub fn case_3() {
        let prices: Vec<i32> = vec![1,2];
        assert_eq!(Solution::max_profit(prices), 1);
    }
}