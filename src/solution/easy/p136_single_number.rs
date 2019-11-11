use crate::Solution;

/// https://leetcode.com/problems/single-number/
///
/// Given a **non-empty** array of integers, every element appears twice except for one. Find that single one.
/// Note: Your algorithm should have a linear runtime complexity. Could you implement it without using extra memory?
///
/// # Example
///
/// > Input: [2, 2, 1]
/// > Output: 1
///
/// > Input: [4, 1, 2, 1, 2]
/// > Output: 4
///
impl Solution {
    pub fn single_number(nums: Vec<i32>) -> i32 {
        nums.iter().fold(0, |acc, num| acc ^ *num)
    }
}

#[test]
fn test_a() {
    assert_eq!(Solution::single_number(vec![2, 2, 1]), 1)
}

#[test]
fn test_b() {
    assert_eq!(Solution::single_number(vec![4, 1, 2, 1, 2]), 4)
}
