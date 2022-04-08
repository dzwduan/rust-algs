/*
 * @lc app=leetcode.cn id=1 lang=rust
 *
 * [1] 两数之和
 */

// @lc code=start

use std::collections::HashMap;

impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut map = HashMap::new();
        let mut v = Vec::new();
        for (i, &num) in nums.iter().enumerate() {
            let r = target - num;
            if let Some(&res) = map.get(&r) {
               v.push(res as i32);
               v.push(i as i32);
               return v;
            } else {
                map.insert(num, i as i32);
            }
        }

        vec![]
    }
}
// @lc code=end
