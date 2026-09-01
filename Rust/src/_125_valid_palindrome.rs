pub struct Solution;
/*
 * @lc app=leetcode id=125 lang=rust
 *
 * [125] Valid Palindrome
 */

// @lc code=start
impl Solution {
    pub fn is_palindrome(s: String) -> bool {
        let chars: Vec<char> = s
            .to_lowercase()
            .chars()
            .collect();

        let mut l = 0;
        let mut r = chars.len() - 1;

        while l < r {
            let cl = chars[l];
            let cr = chars[r];

            if !cl.is_alphanumeric() {
                l += 1;
                continue;
            }
            if !cr.is_alphanumeric() {
                r -= 1;
                continue;
            }

            if chars[l] != chars[r] {
                return false;
            }
            l += 1;
            r -= 1;
        }
        true
    }
}
// @lc code=end
