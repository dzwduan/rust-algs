/*
 * @lc app=leetcode.cn id=11 lang=rust
 *
 * [11] 盛最多水的容器
 */

// @lc code=start
impl Solution {
    pub fn max_area(height: Vec<i32>) -> i32 {
        let (mut l, mut r) = (0, height.len()-1);

        let mut res = 0;

        while l < r {
            let area = std::cmp::min(height[l], height[r]) * (r as i32-l as i32);
            res = std::cmp::max(res, area);
            if height[l] < height[r] {
                l+=1;
            }
            else {
                r-=1;
            }
        }


        res
    }
}
// @lc code=end

