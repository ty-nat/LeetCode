// Definition for singly-linked list.
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    fn new(val: i32) -> Self {
        ListNode { next: None, val }
    }
}
pub struct Solution;

//reference https://leetcode.com/problems/add-two-numbers/discuss/469977/Simple-Rust-solution-less0ms-2.1MBgreater
impl Solution {
    pub fn add_two_numbers(
        l1: Option<Box<ListNode>>,
        l2: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        match (l1, l2) {
            (Some(n), None) => Some(n),
            (None, Some(n)) => Some(n),
            (Some(x), Some(y)) => {
                let total = x.val + y.val;
                let (carry, remain) = if total < 10 {
                    (None, total)
                } else {
                    (Some(Box::new(ListNode::new(1))), total - 10)
                };
                match (carry, remain) {
                    (None, r) => Some(Box::new(ListNode {
                        val: r,
                        next: Solution::add_two_numbers(x.next, y.next),
                    })),
                    (carry, remain) => Some(Box::new(ListNode {
                        val: remain,
                        next: Solution::add_two_numbers(
                            Solution::add_two_numbers(carry, x.next),
                            y.next,
                        ),
                    })),
                }
            }
            _ => None,
        }
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {
        let l1 = Some(Box::new(ListNode {
            val: 2,
            next: Some(Box::new(ListNode {
                val: 4,
                next: Some(Box::new(ListNode::new(3))),
            })),
        }));
        let l2 = Some(Box::new(ListNode {
            val: 5,
            next: Some(Box::new(ListNode {
                val: 6,
                next: Some(Box::new(ListNode::new(4))),
            })),
        }));
        assert_eq!(
            Solution::add_two_numbers(l1, l2),
            Some(Box::new(ListNode {
                val: 7,
                next: Some(Box::new(ListNode {
                    val: 0,
                    next: Some(Box::new(ListNode::new(8))),
                })),
            }))
        )
    }
}
