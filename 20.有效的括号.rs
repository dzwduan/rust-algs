/*
 * @lc app=leetcode.cn id=20 lang=rust
 *
 * [20] 有效的括号
 */

// @lc code=start
impl Solution {
    pub fn is_valid(s: String) -> bool {
        let mut stack = vec!['0'];
        for c in s.chars() {
            match c {
                '('|'{'|'[' => {stack.push(c)},
                ')' => {if stack.pop().unwrap() != '(' { return false}},
                ']' => {if stack.pop().unwrap() != '[' { return false}},
                '}' => {if stack.pop().unwrap() != '{' { return false}},
                _ => (),
            }
        }
        println!("stack.len = {}", stack.len());
        stack.len() == 1
    }
}
// @lc code=end

