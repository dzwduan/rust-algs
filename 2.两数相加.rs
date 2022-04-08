/*
 * @lc app=leetcode.cn id=2 lang=rust
 *
 * [2] 两数相加
 */

// @lc code=start
// Definition for singly-linked list.
// #[derive(PartialEq, Eq, Clone, Debug)]
// pub struct ListNode {
//   pub val: i32,
//   pub next: Option<Box<ListNode>>
// }
//
// impl ListNode {
//   #[inline]
//   fn new(val: i32) -> Self {
//     ListNode {
//       next: None,
//       val
//     }
//   }
// }
impl Solution {
    pub fn add_two_numbers(l1: Option<Box<ListNode>>, l2: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        //转移所有权
        let mut l1 = l1;
        let mut l2 = l2;

        let mut dummy = Some(Box::new(ListNode::new(0)));
        let mut current = dummy.as_mut();
        let mut t = 0;


        while l1.is_some() || l2.is_some() || t>0 {

            if let Some(node) = l1 {
                t += node.val;
                l1 = node.next;
            }

            if let Some(node) = l2 {
                t+= node.val;
                l2 = node.next;
            }

            if let Some(node) = current {
                node.next = Some(Box::new(ListNode::new(t%10)));
                current = node.next.as_mut();
            }

            t/=10;
        }

        dummy.unwrap().next
    }
}
// @lc code=end

