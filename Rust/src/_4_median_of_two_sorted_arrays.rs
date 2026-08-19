pub struct Solution;
/*
 * @lc app=leetcode id=4 lang=rust
 *
 * [4] Median of Two Sorted Arrays
 */

// @lc code=start
impl Solution {
    pub fn find_median_sorted_arrays(nums1: Vec<i32>, nums2: Vec<i32>) -> f64 {
        let mut sorted_array: Vec<i32> = [nums1.clone(), nums2.clone()].concat();

        if sorted_array.len() == 1 {
           return sorted_array[0] as f64;
        }

        sorted_array.sort();


        if sorted_array.len() % 2 != 0 {
            let index = ((sorted_array.len() - 1) / 2) + 1;
            let ans = (sorted_array[index -1]) as f64;
            ans
        } else {
            let n1 = sorted_array[(sorted_array.len() / 2) - 1];
            let n2 = sorted_array[sorted_array.len() / 2];
            let ans = ((n1 + n2) as f64 / 2 as f64) as f64;
            ans
        }
    }
}
// @lc code=end
