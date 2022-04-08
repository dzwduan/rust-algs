/*
 * @lc app=leetcode.cn id=6 lang=rust
 *
 * [6] Z 字形变换
 */

// @lc code=start
impl Solution {
    pub fn convert(s: String, num_rows: i32) -> String {
        let len = s.len();
        let n = num_rows;
        let seq:Vec<char> = s.chars().collect();
        let mut res:Vec<char> = Vec::new();

        if n==1 {
            return s;
        }

        let mut i = 0;

        for i in 0..n {
            if i==0 || i==n-1 {
                let mut j = i;
                while j<(len as i32) {
                    res.push(seq[j as usize]);
                    j+= (2*n-2);
                }
            } else {
                let mut j = i;
                let mut k = 2*n-2-i;

                while j<(len as i32) || k<(len as i32) {

                    if j< (len as i32){
                        res.push(seq[j as usize]);
                        j+=(2*n-2);
                    }

                    if k< (len as i32){
                        res.push(seq[k as usize]);
                        k+=(2*n-2);
                    }
                }
            }
        }

       res.into_iter().collect()
    }
}
// @lc code=end

