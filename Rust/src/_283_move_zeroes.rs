pub struct Solution;
/*
 * @lc app=leetcode id=283 lang=rust
 *
 * [283] Move Zeroes
 */

// @lc code=start
impl Solution {
    // 0, 1, 0, 2, 1, 0, 1, 3, 2, 1, 2, 1
    pub fn move_zeroes(nums: &mut Vec<i32>) {
        let mut p1 = 0;
        let mut p2 = 0;
        while p2 < nums.len() {
            if nums[p1] == 0 && nums[p2] == 0 {
                p2 += 1;
                continue;
            }
            if p1 >= p2 {
                p2 += 1;
                continue;
            }
            if nums[p1] == 0 && nums[p2] != 0 {
                Self::swap(nums, &p1, &p2);
                p1 += 1;
                continue;
            }
            p1 += 1;
        }
    }
    pub fn swap(nums: &mut Vec<i32>, p1: &usize, p2: &usize) {
        let temp = nums[*p1];
        nums[*p1] = nums[*p2];
        nums[*p2] = temp;
    }
}
// @lc code=end
