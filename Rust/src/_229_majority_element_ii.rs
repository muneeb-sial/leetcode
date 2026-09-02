pub struct Solution;
/*
 * @lc app=leetcode id=229 lang=rust
 *
 * [229] Majority Element II
 */

// @lc code=start
impl Solution {
    pub fn majority_element(nums: Vec<i32>) -> Vec<i32> {
        let mut map = std::collections::HashMap::new();
        let mut result = vec![];
        for i in &nums {
            *map.entry(i).or_insert(0) += 1;
        }
        let check: usize = nums.len() / 3;

        result = map
            .iter()
            .filter(|(_, v)| **v > check as i32)
            .map(|(&&k, _)| k)
            .collect();

        result
    }
}
// @lc code=end
