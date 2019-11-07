use crate::Solution;

/// https://leetcode.com/problems/add-binary/
///
/// Given two binary strings, return their sum (also a binary string).
/// The input strings are both **non-empty** and contains only characters `1` or `0`.
///
/// # Example
///
/// > Input: a = "11", b = "1"
/// > Output: "100"
///
/// > Input: a = "1010", b = "1011"
/// > Output: "10101"
///
impl Solution {
    pub fn add_binary(a: String, b: String) -> String {}
}

#[test]
fn test_a() {
    assert_eq!(
        Solution::add_binary("11".to_string(), "1".to_string()),
        "100".to_string()
    )
}

#[test]
fn test_b() {
    assert_eq!(
        Solution::add_binary("1010".to_string(), "1011".to_string()),
        "10101".to_string()
    )
}
