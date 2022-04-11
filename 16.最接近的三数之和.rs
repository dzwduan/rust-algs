/*
 * @lc app=leetcode.cn id=16 lang=rust
 *
 * [16] 最接近的三数之和
 */

// @lc code=start
impl Solution {
    pub fn three_sum_closest(nums: Vec<i32>, target: i32) -> i32 {
        let mut ans:i32 = 0x3f3f3f3f;
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

                if i32::abs((sum - target)) < i32::abs((ans - target)) {
                    ans = sum;
                }

                if sum > target {
                    r-=1;
                    continue;
                }

                if sum < target {
                    l +=1;
                    continue;
                }

                return target;
            }


        }

        ans
    }
}
// @lc code=end

