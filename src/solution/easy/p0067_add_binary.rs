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
            if c == 3 {
                chars_c.push('1');
                need_plus = c - 2
            } else if c == 2 {
                chars_c.push('0');
                need_plus = c - 1
            } else {
                chars_c.push((c + 48) as char)
            }
        }
        chars_c.iter().rev().collect::<String>()
    }

    pub fn add_binary_another(a: String, b: String) -> String {
        let mut res: Vec<char> = vec![];
        let v_a: Vec<char> = a.chars().collect();
        let v_b: Vec<char> = b.chars().collect();
        let mut carry = 0;
        let mut idx_a = a.len();
        let mut idx_b = b.len();
        while idx_a > 0 || idx_b > 0 || carry == 1 {
            if idx_a > 0 {
                idx_a -= 1;
                carry += v_a[idx_a] as u8 - 48;
            }
            if idx_b > 0 {
                idx_b -= 1;
                carry += v_b[idx_b] as u8 - 48;
            }
            res.push((carry % 2 + 48) as char);
            carry /= 2;
        }
        res.iter().rev().collect::<String>()
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

#[test]
fn test_d() {
    assert_eq!(
        Solution::add_binary("1111".to_string(), "1111".to_string()),
        "11110".to_string()
    )
}
