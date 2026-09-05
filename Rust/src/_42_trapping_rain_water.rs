pub struct Solution;
/*
 * @lc app=leetcode id=42 lang=rust
 *
 * [42] Trapping Rain Water
 */

// @lc code=start
impl Solution {
    pub fn trap(height: Vec<i32>) -> i32 {
        if height.len() == 0 {
            return 0;
        }

        let mut max_lefts = vec![0; height.len()];
        let mut max_rights = vec![0; height.len()];
        let mut water = 0;

        max_lefts[0] = height[0];
        for i in 1..height.len() {
            max_lefts[i] = std::cmp::max(max_lefts[i - 1], height[i]);
        }

        max_rights[height.len() - 1] = height[height.len() - 1];
        for i in (0..height.len() - 1).rev() {
            max_rights[i] = std::cmp::max(max_rights[i + 1], height[i]);
        }

        for i in 0..height.len() {
            water += std::cmp::min(max_lefts[i], max_rights[i]) - height[i];
        }
        water
    }
}
// @lc code=end
