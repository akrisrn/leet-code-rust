use crate::Solution;

/// https://leetcode.com/problems/search-insert-position/
///
/// Given a sorted array and a target value, return the index if the target is found.
/// If not, return the index where it would be if it were inserted in order.
/// You may assume no duplicates in the array.
///
/// # Example
///
/// > Input: [1, 3, 5, 6], 5
/// > Output: 2
///
/// > Input: [1, 3, 5, 6], 2
/// > Output: 1
///
/// > Input: [1, 3, 5, 6], 7
/// > Output: 4
///
/// > Input: [1, 3, 5, 6], 0
/// > Output: 0
///
impl Solution {
    pub fn search_insert(nums: Vec<i32>, target: i32) -> i32 {
        let mut index = 0;
        for (i, &num) in nums.iter().enumerate() {
            if num == target {
                return i as i32;
            } else if num < target {
                index = i as i32 + 1
            } else if num > target {
                return index;
            }
        }
        index
    }
}

#[test]
fn test_a() {
    assert_eq!(Solution::search_insert(vec![1, 3, 5, 6], 5), 2)
}

#[test]
fn test_b() {
    assert_eq!(Solution::search_insert(vec![1, 3, 5, 6], 2), 1)
}

#[test]
fn test_c() {
    assert_eq!(Solution::search_insert(vec![1, 3, 5, 6], 7), 4)
}

#[test]
fn test_d() {
    assert_eq!(Solution::search_insert(vec![1, 3, 5, 6], 0), 0)
}
