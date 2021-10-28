use std::collections::HashMap;

use crate::Solution;

/// https://leetcode.com/problems/two-sum/
///
/// Given an array of integers, return **indices** of the two numbers such that they add up to a specific target.
/// You may assume that each input would have **exactly** one solution, and you may not use the same element twice.
///
/// # Example
///
/// > Given nums = [2, 7, 11, 15], target = 9,
/// > Because nums[0] + nums[1] = 2 + 7 = 9,
/// > return [0, 1].
///
impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut com_nums = vec![];
        for num in &nums {
            com_nums.push(target - *num)
        }
        let mut result_nums = vec![];
        for (i, num) in nums.iter().enumerate() {
            if com_nums.contains(num) {
                result_nums.push(i as i32)
            }
        }
        if result_nums.len() == 3 {
            result_nums.retain(|&i| nums[i as usize] * 2 != target)
        }
        result_nums
    }

    pub fn two_sum_0ms(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut seen = HashMap::new();
        for (i, num) in nums.iter().enumerate() {
            if seen.contains_key(num) {
                return vec![seen[num] as i32, i as i32];
            } else {
                seen.insert(target - *num, i);
            }
        }
        return vec![];
    }
}

#[test]
fn test_a() {
    assert_eq!(Solution::two_sum(vec![2, 7, 11, 15], 9), vec![0, 1]);
}

#[test]
fn test_b() {
    assert_eq!(Solution::two_sum(vec![-12, 31, 9, -25], 6), vec![1, 3]);
}

#[test]
fn test_c() {
    assert_eq!(Solution::two_sum(vec![-9, 0, -2, -15], -11), vec![0, 2]);
}

#[test]
fn test_d() {
    assert_eq!(Solution::two_sum(vec![3, 2, 4], 6), vec![1, 2]);
}

#[test]
fn test_e() {
    // > and you may not use the same element twice.
    // WRRRRRRRRRY.jpg
    assert_eq!(Solution::two_sum(vec![3, 3], 6), vec![0, 1]);
}
