pub struct Solution;
/*
 * @lc app=leetcode id=75 lang=rust
 *
 * [75] Sort Colors
 */

// @lc code=start
impl Solution {
    pub fn sort_colors(nums: &mut Vec<i32>) {
        let mut start_pointer = 0;
        let mut end_pointer = (nums.len() - 1) as i32;
        let mut array_pointer = 0;

        while array_pointer <= end_pointer {
            if nums[array_pointer as usize] == 0 {
                Self::swap(start_pointer, array_pointer, nums);
                start_pointer += 1;
            } else if nums[array_pointer as usize] == 2 {
                Self::swap(array_pointer, end_pointer, nums);
                end_pointer -= 1;
                array_pointer -= 1;
            }
            array_pointer += 1;
        }
    }

    pub fn swap(i: i32, j: i32, nums: &mut Vec<i32>) {
        let temp = nums[i as usize];
        nums[i as usize] = nums[j as usize];
        nums[j as usize] = temp;
    }
}
// @lc code=end
