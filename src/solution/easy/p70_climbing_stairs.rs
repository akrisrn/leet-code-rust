use crate::Solution;

/// https://leetcode.com/problems/climbing-stairs/
///
/// You are climbing a stair case. It takes n steps to reach to the top.
/// Each time you can either climb 1 or 2 steps. In how many distinct ways can you climb to the top?
/// Note: Given n will be a positive integer.
///
/// # Example
///
/// > Input: 2
/// > Output: 2
/// > Explanation: There are two ways to climb to the top.
/// > 1. 1 step + 1 step
/// > 2. 2 steps
///
/// > Input: 3
/// > Output: 3
/// > Explanation: There are three ways to climb to the top.
/// > 1. 1 step + 1 step + 1 step
/// > 2. 1 step + 2 steps
/// > 3. 2 steps + 1 step
///
impl Solution {
    pub fn climb_stairs(n: i32) -> i32 {}
}

#[test]
fn test_a() {
    assert_eq!(Solution::climb_stairs(2), 2)
}

#[test]
fn test_b() {
    assert_eq!(Solution::climb_stairs(3), 3)
}
