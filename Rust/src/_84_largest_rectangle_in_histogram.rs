pub struct Solution;

/*
 * @lc app=leetcode id=84 lang=rust
 *
 * [84] Largest Rectangle in Histogram
 */

// @lc code=start
impl Solution {
    pub fn largest_rectangle_area(heights: Vec<i32>) -> i32 {
        if heights.is_empty() {
            return 0;
        }

        let mut area = 0;
        let mut stack: Vec<i32> = vec![0];

        for idx in 1..heights.len() {
            let curr = heights[idx];

            while !stack.is_empty() && heights[*stack.last().unwrap() as usize] > curr {
                let popped = stack.pop().unwrap();
                let height = heights[popped as usize];

                let width: i32 = match stack.last() {
                    None => idx as i32,
                    Some(&last) => idx as i32 - last - 1,
                };

                area = area.max(width * height);
            }

            stack.push(idx as i32);
        }

        // Process remaining bars.
        let idx = heights.len();

        while !stack.is_empty() {
            let popped = stack.pop().unwrap();
            let height = heights[popped as usize];

            let width: i32 = match stack.last() {
                None => idx as i32,
                Some(&last) => idx as i32 - last - 1,
            };

            area = area.max(width * height);
        }

        area
    }
}
// @lc code=end
