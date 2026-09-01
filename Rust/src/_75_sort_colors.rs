pub struct Solution; 
/*
 * @lc app=leetcode id=75 lang=rust
 *
 * [75] Sort Colors
 */

// @lc code=start
impl Solution {
    pub fn sort_colors(nums: &mut Vec<i32>) {
        nums.sort_unstable();
    }
}
// @lc code=end

