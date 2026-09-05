pub struct Solution;
/*
 * @lc app=leetcode id=167 lang=rust
 *
 * [167] Two Sum II - Input Array Is Sorted
 */

// @lc code=start
impl Solution {
    pub fn two_sum(numbers: Vec<i32>, target: i32) -> Vec<i32> {
        let mut idx = vec![];
        let mut l = 0;
        let mut r = numbers.len() - 1;

        while l < r {
            let sum = numbers[l] + numbers[r];
            if sum == target {
                idx.push((l + 1) as i32);
                idx.push((r + 1) as i32);
                break;
            }
            if sum > target {
                r -= 1;
            }
            if sum < target {
                l += 1;
            }
        }

        idx
    }
}
// @lc code=end
