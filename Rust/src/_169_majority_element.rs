pub struct Solution;

/*
 * @lc app=leetcode id=169 lang=rust
 *
 * [169] Majority Element
 */

// @lc code=start
impl Solution {
    pub fn majority_element(nums: Vec<i32>) -> i32 {
        let mut count = 0;
        let mut result = 0;
        for i in nums {
            if count == 0 {
                result = i;
            }
            if result == i { count += 1 } else { count -= 1 }
        }
        result
    }
}
// @lc code=end
