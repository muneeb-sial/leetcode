use std::collections::HashSet;

pub struct Solution;

/*
 * @lc app=leetcode id=217 lang=rust
 *
 * [217] Contains Duplicate
 */

// @lc code=start
impl Solution {
    pub fn contains_duplicate(nums: Vec<i32>) -> bool {
        let set: std::collections::HashSet<_> = nums.iter().collect();
        set.len() != nums.len()
    }
}
// @lc code=end
