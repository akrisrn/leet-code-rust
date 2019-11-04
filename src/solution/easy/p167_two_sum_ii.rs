use crate::Solution;
use std::collections::HashMap;

/// https://leetcode.com/problems/two-sum-ii-input-array-is-sorted/
///
/// Given an array of integers that is already **sorted in ascending order**,
/// find two numbers such that they add up to a specific target number.
/// The function twoSum should return indices of the two numbers such that they add up to the target,
/// where index1 must be less than index2.
///
/// # Note
///
/// - Your returned answers (both index1 and index2) are not zero-based.
/// - You may assume that each input would have exactly one solution and you may not use the same element twice.
///
/// # Example
///
/// > Input: numbers = [2, 7, 11, 15], target = 9
/// > Output: [1, 2]
/// > Explanation: The sum of 2 and 7 is 9. Therefore index1 = 1, index2 = 2.
///
impl Solution {
    pub fn two_sum_ii(numbers: Vec<i32>, target: i32) -> Vec<i32> {
        let mut map = HashMap::new();
        for (i, number) in numbers.iter().enumerate() {
            if map.contains_key(number) {
                return vec![map[number] as i32, (i + 1) as i32];
            } else {
                map.insert(target - *number, i + 1);
            }
        }
        vec![]
    }
}

#[test]
fn test_a() {
    assert_eq!(Solution::two_sum_ii(vec![2, 7, 11, 15], 9), vec![1, 2]);
}
