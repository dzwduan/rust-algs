/*
 * @lc app=leetcode.cn id=7 lang=rust
 *
 * [7] 整数反转
 */

// @lc code=start
impl Solution {
    pub fn reverse(x: i32) -> i32 {
        let mut r = 0;
        let mut x = x;

        while x != 0 {
            if r > 0 && r > i32::MAX / 10 {
                return 0;
            }
            if r < 0 && r < i32::MIN / 10 {
                return 0;
            }
            r = r * 10 + x % 10;
            x /= 10;
        }

        r
    }
}
// @lc code=end
