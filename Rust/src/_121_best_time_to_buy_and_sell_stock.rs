pub struct Solution;
/*
 * @lc app=leetcode id=121 lang=rust
 *
 * [121] Best Time to Buy and Sell Stock
 */

// @lc code=start
impl Solution {
    pub fn max_profit(prices: Vec<i32>) -> i32 {
        let mut l = 0;
        let mut r = 0;
        let mut profit = 0;

        while r < prices.len() {
            if prices[l] < prices[r] {
                profit = std::cmp::max(profit, prices[r] - prices[l]);
            } else {
                l = r
            }
            r += 1;
        }
        profit
    }
}
// @lc code=end
