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
    pub fn add_binary(a: String, b: String) -> String {
        let bytes_a = a.into_bytes();
        let bytes_b = b.into_bytes();
        let mut chars_c = vec![];
        let mut i_a = bytes_a.len();
        let mut i_b = bytes_b.len();
        let mut need_plus = 0;
        while i_a > 0 || i_b > 0 || need_plus > 0 {
            let mut c = 0;
            if i_a > 0 {
                c += bytes_a[i_a - 1] - 48;
                i_a -= 1;
            }
            if i_b > 0 {
                c += bytes_b[i_b - 1] - 48;
                i_b -= 1;
            }
            if need_plus > 0 {
                c += need_plus;
                need_plus = 0;
            }
            if c > 1 {
                chars_c.push('0');
                need_plus = c - 1
            } else {
                chars_c.push((c + 48) as char)
            }
        }
        chars_c.iter().rev().collect::<String>()
    }
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

#[test]
fn test_c() {
    assert_eq!(
        Solution::add_binary("111111111".to_string(), "1".to_string()),
        "1000000000".to_string()
    )
}
