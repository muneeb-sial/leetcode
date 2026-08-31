pub struct Solution;
/*
 * @lc app=leetcode id=128 lang=rust
 *
 * [128] Longest Consecutive Sequence
 */

// @lc code=start
impl Solution {
    pub fn longest_consecutive(nums: Vec<i32>) -> i32 {
        let set: std::collections::HashSet<_> = nums.iter().collect();
        let mut counter = 0;

        for i in set.clone() {
            let back_check = i.clone() - 1;
            
            if !set.contains(&back_check) {
                let mut length = 0;
                let mut front_check = i.clone() + length;
                while set.contains(&front_check) {
                    length += 1;
                    front_check += 1;
                }
                counter = std::cmp::max(length, counter);
            }
        }
        counter
    }
}
// @lc code=end
