use crate::Solution;

/// https://leetcode.com/problems/reverse-integer/
///
/// Given a 32-bit signed integer, reverse digits of an integer.
///
/// # Note
///
/// Assume we are dealing with an environment which could only store integers within the 32-bit signed integer range:
/// [−2^31,  2^31 − 1].
/// For the purpose of this problem, assume that your function returns 0 when the reversed integer overflows.
///
/// # Example
///
/// > Input: 123
/// > Output: 321
///
/// > Input: -123
/// > Output: -321
///
/// > Input: 120
/// > Output: 21
///
impl Solution {
    pub fn reverse(x: i32) -> i32 {
        let mut bytes = x.to_string().into_bytes();
        bytes.reverse();
        let len = bytes.len();
        if bytes[len - 1] == b'-' {
            bytes.remove(len - 1);
            bytes.insert(0, b'-');
        }
        match String::from_utf8(bytes).unwrap().parse::<i32>() {
            Ok(x) => x,
            Err(_) => 0,
        }
    }
}

#[test]
fn test_a() {
    assert_eq!(Solution::reverse(123), 321);
}

#[test]
fn test_b() {
    assert_eq!(Solution::reverse(-123), -321);
}

#[test]
fn test_c() {
    assert_eq!(Solution::reverse(120), 21);
}

#[test]
fn test_d() {
    assert_eq!(Solution::reverse(2147483647), 0);
}

#[test]
fn test_e() {
    assert_eq!(Solution::reverse(-2147483648), 0);
}

#[test]
fn test_f() {
    assert_eq!(Solution::reverse(2063847412), 2147483602);
}

#[test]
fn test_g() {
    assert_eq!(Solution::reverse(-2063847412), -2147483602);
}
