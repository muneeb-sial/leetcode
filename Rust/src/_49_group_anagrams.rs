pub struct Solution;
/*
 * @lc app=leetcode id=49 lang=rust
 *
 * [49] Group Anagrams
 */

// @lc code=start
impl Solution {

     pub fn group_anagrams(strs: Vec<String>) -> Vec<Vec<String>> {
        let mut groups: std::collections::HashMap<String, Vec<String>> = std::collections::HashMap::new();

        for str in strs {
            let mut chars: Vec<char> = str.chars().collect();
            chars.sort_unstable();

            let key: String = chars.into_iter().collect();

            groups.entry(key).or_default().push(str);
        }

        groups.into_values().collect()
    }
}
// @lc code=end
