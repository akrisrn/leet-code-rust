use crate::Solution;

/// https://leetcode.com/problems/best-time-to-buy-and-sell-stock/
///
/// Say you have an array for which the i^th element is the price of a given stock on day i.
/// If you were only permitted to complete at most one transaction (i.e., buy one and sell one share of the stock),
/// design an algorithm to find the maximum profit.
/// Note that you cannot sell a stock before you buy one.
///
/// # Example
///
/// > Input: [7, 1, 5, 3, 6, 4]
/// > Output: 5
/// > Explanation: Buy on day 2 (price = 1) and sell on day 5 (price = 6), profit = 6 - 1 = 5.
/// >              Not 7 - 1 = 6, as selling price needs to be larger than buying price.
///
/// > Input: [7, 6, 4, 3, 1]
/// > Output: 0
/// > Explanation: In this case, no transaction is done, i.e. max profit = 0.
///
impl Solution {
    pub fn max_profit(prices: Vec<i32>) -> i32 {}
}

#[test]
fn test_a() {
    assert_eq!(Solution::max_profit(vec![7, 1, 5, 3, 6, 4]), 5)
}

#[test]
fn test_b() {
    assert_eq!(Solution::max_profit(vec![7, 6, 4, 3, 1]), 0)
}
