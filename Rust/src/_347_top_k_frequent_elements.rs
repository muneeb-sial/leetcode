pub struct Solution;
/*
 * @lc app=leetcode id=347 lang=rust
 *
 * [347] Top K Frequent Elements
 */

// @lc code=start
impl Solution {
    pub fn top_k_frequent(nums: Vec<i32>, k: i32) -> Vec<i32> {
        let mut  map = std::collections::HashMap::new();
        for i in nums {
            *map.entry(i).or_insert(0) += 1;
        }
        let mut sorted: Vec<(&i32, &i32)> = map.iter().collect();

        // Sort by value (ascending)
        sorted.sort_by(|a, b| b.1.cmp(a.1));
        let top_k: Vec<i32> = sorted
            .iter()
            .take(k as usize)
            .map(|(key, _)| **key)
            .collect();
        top_k
    }
}
// @lc code=end
