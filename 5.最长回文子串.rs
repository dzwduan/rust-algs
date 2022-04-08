/*
 * @lc app=leetcode.cn id=5 lang=rust
 *
 * [5] 最长回文子串
 */

// @lc code=start
impl Solution {
    pub fn longest_palindrome(s: String) -> String {
        let n = s.len() as isize;
        if n==0 {
            return s;
        }
        let seq: Vec<char> = s.chars().collect();
        let mut i:isize = 0;
        let (mut max_l, mut max_r) = (0, 0);
    
        while i < n as isize {
            let mut l:isize = i - 1;
            let mut r:isize = i + 1;
    
            while l >= 0 && r < n && seq[l as usize] == seq[r as usize] {
                l -= 1;
                r += 1;
            }
            if (max_r - max_l) < (r - l) {
                max_l = l;
                max_r = r;
            }
    
            l = i;   r = i + 1;
    
            while l >= 0 && r < n && seq[l as usize] == seq[r as usize] {
                l -= 1;
                r += 1;
            }
            if (max_r - max_l) < (r - l) {
                max_l = l;
                max_r = r;
            }
    
            i += 1;
        }
    
        // println!("{} {}", max_l, max_r);
    
        s[(max_l+1)    as usize..max_r as usize].to_owned()
    }
}
// @lc code=end
