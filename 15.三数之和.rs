/*
 * @lc app=leetcode.cn id=15 lang=rust
 *
 * [15] 三数之和
 */

// @lc code=start
impl Solution {
    pub fn three_sum(nums: Vec<i32>) -> Vec<Vec<i32>> {
        let mut ret = Vec::new();
        let n = nums.len();
        let mut nums = nums.clone();
        nums.sort();

        for i in 0..n {
            if i!=0 && nums[i] == nums[i-1] {
                continue;
            }

            let mut l = i+1;
            let mut r = n-1;

            while l < r {
                let mut sum = nums[i] + nums[l] + nums[r];

                if sum > 0 {
                    r-=1;
                    continue;
                }

                if sum < 0 {
                    l +=1;
                    continue;
                }

                ret.push(vec![nums[i], nums[l], nums[r]]);

                loop {
                    l+=1;
                    if !(l < r && nums[l] == nums[l-1]) {break;}
                }

                loop {
                    r-=1;
                    if !(l < r && nums[r] == nums[r+1]) {break;}
                }
            }


        }

        ret
    }
}
// @lc code=end

