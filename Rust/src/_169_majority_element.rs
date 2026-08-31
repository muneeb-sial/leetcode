pub struct Solution;

/*
 * @lc app=leetcode id=169 lang=rust
 *
 * [169] Majority Element
 */

// @lc code=start
impl Solution {
    pub fn majority_element(nums: Vec<i32>) -> i32 {
        let mut h_map = std::collections::HashMap::new();
        for i in nums {
            *h_map.entry(i).or_insert(0) += 1;
        }

        let max_key = h_map.iter().max_by_key(|(_, v)| *v).map(|(k, _)| *k);
        max_key.unwrap()
    }
}
// @lc code=end
