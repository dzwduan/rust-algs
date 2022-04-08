/*
 * @lc app=leetcode.cn id=3 lang=rust
 *
 * [3] 无重复字符的最长子串
 */



// @lc code=start

use std::collections::HashMap;
impl Solution {
    
    pub fn length_of_longest_substring(s: String) -> i32 {
        let mut heap = HashMap::new();
        let mut res = 0;
    
        let mut l = 0;
    
        for (r, c) in s.char_indices() {
            if let Some(end) = heap.insert(c,r) {
                l = usize::max(l, end+1);
            }
    
            res = usize::max(r-l+1, res);
        }
    
        res as i32
    }
}
// @lc code=end

