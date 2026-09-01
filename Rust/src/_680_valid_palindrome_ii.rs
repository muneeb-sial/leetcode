pub struct Solution;
/*
 * @lc app=leetcode id=680 lang=rust
 *
 * [680] Valid Palindrome II
 */

// @lc code=start
impl Solution {
    pub fn valid_palindrome(s: String) -> bool {
        let chars: Vec<char> = s.to_lowercase().chars().collect();

        let mut l = 0;
        let mut r = chars.len() - 1;

        while l < r {
            if chars[l] != chars[r] {
                let lchars: String = chars[l + 1..r + 1].iter().collect();
                let rchars: String = chars[l..r].iter().collect();
                let rlchars: String = lchars.chars().rev().collect();
                let rrchars: String = rchars.chars().rev().collect();

                return lchars == rlchars || rchars == rrchars
            }
            l += 1;
            r -= 1;
        }
        true
    }
}
// @lc code=end
