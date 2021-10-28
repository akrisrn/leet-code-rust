use crate::Solution;

/// https://leetcode.com/problems/sqrtx/
///
/// Implement `int sqrt(int x)`.
/// Compute and return the square root of x, where x is guaranteed to be a non-negative integer.
/// Since the return type is an integer,
/// the decimal digits are truncated and only the integer part of the result is returned.
///
/// # Example
///
/// > Input: 4
/// > Output: 2
///
/// > Input: 8
/// > Output: 2
/// > Explanation: The square root of 8 is 2.82842..., and since
/// >              the decimal part is truncated, 2 is returned.
///
impl Solution {
    pub fn my_sqrt(x: i32) -> i32 {
        (x as f64).sqrt() as i32
    }
}

#[test]
fn test_a() {
    assert_eq!(Solution::my_sqrt(4), 2)
}

#[test]
fn test_b() {
    assert_eq!(Solution::my_sqrt(8), 2)
}
