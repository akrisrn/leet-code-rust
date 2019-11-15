use crate::Solution;

/// https://leetcode.com/problems/merge-sorted-array/
///
/// Given two sorted integer arrays nums1 and nums2, merge nums2 into nums1 as one sorted array.
///
/// Note:
///
/// - The number of elements initialized in nums1 and nums2 are m and n respectively.
/// - You may assume that nums1 has enough space (size that is greater or equal to m + n)
///   to hold additional elements from nums2.
///
/// # Example
///
/// > Input:
/// > nums1 = [1, 2, 3, 0, 0, 0], m = 3
/// > nums2 = [2, 5, 6],          n = 3
/// >
/// > Output: [1, 2, 2, 3, 5, 6]
///
impl Solution {
    pub fn merge(nums1: &mut Vec<i32>, m: i32, nums2: &mut Vec<i32>, n: i32) -> Vec<i32> {}
}

#[test]
fn test_a() {
    assert_eq!(Solution::merge(&mut vec![1, 2, 3, 0, 0, 0], 3, &mut vec![2, 5, 6], 3), vec![1, 2, 2, 3, 5, 6])
}
