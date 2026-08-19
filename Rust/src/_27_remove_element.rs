pub struct Solution;
/*
 * @lc app=leetcode id=27 lang=rust
 *
 * [27] Remove Element
 */

// @lc code=start
impl Solution {
    pub fn remove_element(nums: &mut Vec<i32>, val: i32) -> i32 {

        let length_of_array = nums.len();
        let mut num_to_time_value_appears = 0;
        for index in 0..nums.len(){
            if nums[index] == val{
                num_to_time_value_appears+=1;
                // nums[index]=
                continue;
            }
            // nums[index] = element.clone();

        }
     return length_of_array as i32 - num_to_time_value_appears;
    }
}
// @lc code=end

