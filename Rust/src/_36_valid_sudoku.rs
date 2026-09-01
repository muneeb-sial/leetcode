pub struct Solution;
/*
 * @lc app=leetcode id=36 lang=rust
 *
 * [36] Valid Sudoku
 */

// @lc code=start
impl Solution {
    pub fn is_valid_sudoku(board: Vec<Vec<char>>) -> bool {
        let mut cols: std::collections::HashMap<i32, std::collections::HashSet<char>> = std::collections::HashMap::new();
        let mut rows: std::collections::HashMap<i32, std::collections::HashSet<char>> = std::collections::HashMap::new();
        let mut squares: std::collections::HashMap<i32, std::collections::HashSet<char>> = std::collections::HashMap::new();

        for r in 0..9 {
            for c in 0..9 {
                let val = board[r][c];
                if val == '.' {
                    continue;
                }

                let square_key = ((r / 3) * 3 + c / 3) as i32;

                let col_has = cols.get(&(c as i32)).map_or(false, |s| s.contains(&val));
                let row_has = rows.get(&(r as i32)).map_or(false, |s| s.contains(&val));
                let sq_has = squares.get(&square_key).map_or(false, |s| s.contains(&val));

                if col_has || row_has || sq_has {
                    return false;
                }

                cols.entry(c as i32)
                    .or_insert_with(std::collections::HashSet::new)
                    .insert(val);
                rows.entry(r as i32)
                    .or_insert_with(std::collections::HashSet::new)
                    .insert(val);
                squares
                    .entry(square_key)
                    .or_insert_with(std::collections::HashSet::new)
                    .insert(val);
            }
        }

        true
    }
}
// @lc code=end
