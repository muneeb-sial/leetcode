pub struct Solution;
/*
 * @lc app=leetcode id=705 lang=rust
 *
 * [705] Design HashSet
 */

// @lc code=start
pub struct MyHashSet {
    array: Vec<bool>,
}

impl MyHashSet {
    
    pub fn new() -> Self {
        MyHashSet {
            array: vec![false; 1_000_001],
        }
    }

    pub fn add(&mut self, key: i32) {
        self.array[key as usize] = true;
    }

    pub fn remove(&mut self, key: i32) {
        self.array[key as usize] = false;
    }

    pub fn contains(&self, key: i32) -> bool {
        self.array[key as usize]
    }
}
// @lc code=end