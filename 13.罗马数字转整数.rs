/*
 * @lc app=leetcode.cn id=13 lang=rust
 *
 * [13] 罗马数字转整数
 */

// @lc code=start
use std::collections::HashMap;
impl Solution {
    pub fn roman_to_int(s: String) -> i32 {
        let map = ['I', 'V', 'X', 'L', 'C', 'D', 'M'].iter().zip([1, 5, 10, 50, 100, 500, 1000]).collect::<HashMap<_,_>>();

        s.chars().rev().fold(0, |acc, x| acc + if acc > 4*map[&x] {-map[&x]} else {map[&x]})
    }
}
// @lc code=end

