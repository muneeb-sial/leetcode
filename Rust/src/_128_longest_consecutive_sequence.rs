pub struct Solution;
/*
 * @lc app=leetcode id=128 lang=rust
 *
 * [128] Longest Consecutive Sequence
 */

// @lc code=start
impl Solution {
    pub fn longest_consecutive(nums: Vec<i32>) -> i32 {
        let set: std::collections::HashSet<i32> = nums.into_iter().collect();
        let mut longest = 0;

        for &n in &set {
            if !set.contains(&(n - 1)) {
                let mut length = 1;
                while set.contains(&(n + length)) {
                    length += 1;
                }
                longest = longest.max(length);
            }
        }

        longest
    }
}
// @lc code=end
