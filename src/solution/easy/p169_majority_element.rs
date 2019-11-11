use crate::Solution;
use std::collections::HashMap;

/// https://leetcode.com/problems/majority-element/
///
/// Given an array of size n, find the majority element.
/// The majority element is the element that appears **more than** `⌊ n/2 ⌋` times.
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
    pub fn majority_element(nums: Vec<i32>) -> i32 {
        if nums.is_empty() {
            return 0;
        }
        let mut maj = nums[0];
        let mut map = HashMap::new();
        for num in &nums {
            if map.contains_key(num) {
                *map.get_mut(num).unwrap() += 1;
                if map[num] > nums.len() / 2 {
                    maj = *num;
                    break;
                }
            } else {
                map.insert(num, 1);
            }
        }
        maj
    }
}

#[test]
fn test_a() {
    assert_eq!(Solution::majority_element(vec![3, 2, 3]), 3)
}

#[test]
fn test_b() {
    assert_eq!(Solution::majority_element(vec![2, 2, 1, 1, 1, 2, 2]), 2)
}

#[test]
fn test_c() {
    assert_eq!(Solution::majority_element(vec![1]), 1)
}
