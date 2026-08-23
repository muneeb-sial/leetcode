pub struct Solution;
/*
 * @lc app=leetcode id=242 lang=rust
 *
 * [242] Valid Anagram
 */

// @lc code=start
impl Solution {
    pub fn is_anagram(s: String, t: String) -> bool {
        if s.len() != t.len(){
           return false
        }
        let mut s_map = std::collections::HashMap::new();
        let mut t_map = std::collections::HashMap::new();
        for ch in s.chars() {
            *s_map.entry(ch).or_insert(0) += 1;
        }
        for ch in t.chars() {
            *t_map.entry(ch).or_insert(0) += 1;
        }

        s_map == t_map

    }
}
// @lc code=end
