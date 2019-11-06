use crate::Solution;

/// https://leetcode.com/problems/maximum-subarray/
///
/// Given an integer array `nums`,
/// find the contiguous subarray (containing at least one number) which has the largest sum and return its sum.
///
/// # Follow up
///
/// If you have figured out the O(n) solution,
/// try coding another solution using the divide and conquer approach, which is more subtle.
///
/// # Example
///
/// > Input: [-2, 1, -3, 4, -1, 2, 1, -5, 4],
/// > Output: 6
/// > Explanation: [4, -1, 2, 1] has the largest sum = 6.
///
impl Solution {
    pub fn max_sub_array(nums: Vec<i32>) -> i32 {
        if nums.is_empty() {
            return 0;
        }
        let mut max = nums[0];
        let mut cur = 0;
        for num in nums {
            let sum = cur + num;
            if sum <= 0 {
                cur = 0;
            } else {
                cur = sum;
            }
            if sum > max {
                max = sum;
            }
        }
        max
    }
}

#[test]
fn test_a() {
    assert_eq!(Solution::max_sub_array(vec![-2, 1, -3, 4, -1, 2, 1, -5, 4]), 6)
}

#[test]
fn test_b() {
    assert_eq!(Solution::max_sub_array(vec![-1]), -1)
}

#[test]
fn test_c() {
    assert_eq!(Solution::max_sub_array(vec![]), 0)
}
