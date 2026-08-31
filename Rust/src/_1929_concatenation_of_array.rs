pub struct Solution;
/*
 * @lc app=leetcode id=1929 lang=rust
 *
 * [1929] Concatenation of Array
 */

// @lc code=start
impl Solution {
    pub fn get_concatenation(mut nums: Vec<i32>) -> Vec<i32> {
        
        nums.reserve(nums.len());
        nums.extend_from_within(..);
        nums
    }
}
// @lc code=end
