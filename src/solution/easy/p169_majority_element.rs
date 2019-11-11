use crate::Solution;

/// https://leetcode.com/problems/majority-element/
///
/// Given an array of size n, find the majority element.
/// The majority element is the element that appears **more than** ⌊ n/2 ⌋ times.
/// You may assume that the array is non-empty and the majority element always exist in the array.
///
/// # Example
///
/// > Input: [3, 2, 3]
/// > Output: 3
///
/// > Input: [2, 2, 1, 1, 1, 2, 2]
/// > Output: 2
///
impl Solution {
    pub fn majority_element(nums: Vec<i32>) -> i32 {}
}

#[test]
fn test_a() {
    assert_eq!(Solution::majority_element(vec![3, 2, 3]), 3)
}

#[test]
fn test_b() {
    assert_eq!(Solution::majority_element(vec![2, 2, 1, 1, 1, 2, 2]), 2)
}
