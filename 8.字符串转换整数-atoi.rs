/*
 * @lc app=leetcode.cn id=8 lang=rust
 *
 * [8] 字符串转换整数 (atoi)
 */

// @lc code=start
impl Solution {
    pub fn my_atoi(s: String) -> i32 {
        //全程使用i64
        //辨别空格，非十进制

        let mut neg = false;
        let mut res = 0i64;

        for (i, n) in s.trim().chars().enumerate() {
            if n == '+' && i == 0 {
                neg = false;
                continue;
            }
            if n == '-' && i == 0 {
                neg = true;
                continue;
            }

            if let Some(y) = n.to_digit(10) {
                res = 10i64 * res + y as i64;
                println!("res = {}", res);
            } else {
                break;
            }

            if !neg && res > i32::MAX as i64 {
                return i32::MAX;
            } else if neg && -res < i32::MIN as i64 {
                return i32::MIN;
            }
        }

        println!("### res = {} ###", res);

        if neg {
            -res as i32
        } else {
            res as i32
        }
    }
}
// @lc code=end
