use crate::Solution;

/// https://leetcode.com/problems/palindrome-number/
///
/// Determine whether an integer is a palindrome.
/// An integer is a palindrome when it reads the same backward as forward.
///
/// # Follow up
///
/// Could you solve it without converting the integer to a string?
///
/// # Example
///
/// > Input: 121
/// > Output: true
///
/// > Input: -121
/// > Output: false
/// > Explanation: From left to right, it reads -121. From right to left, it becomes 121-.
/// > Therefore it is not a palindrome.
///
/// > Input: 10
/// > Output: false
/// > Explanation: Reads 01 from right to left. Therefore it is not a palindrome.
///
impl Solution {
    pub fn is_palindrome(x: i32) -> bool {
        let mut bytes = x.to_string().into_bytes();
        bytes.reverse();
        String::from_utf8(bytes).unwrap() == x.to_string()
    }

    pub fn is_palindrome_0ms(x: i32) -> bool {
        if x < 0 {
            return false;
        }
        let digits = (x as f64).log10() as u32 + 1;
        for i in 0..(digits / 2) {
            let left = x / (10 as i32).pow(digits - i - 1) % 10;
            let right = x / (10 as i32).pow(i) % 10;
            if left != right {
                return false;
            }
        }
        true
    }
}

#[test]
fn test_a() {
    assert!(Solution::is_palindrome(121));
}

#[test]
fn test_b() {
    assert!(!Solution::is_palindrome(-121));
}

#[test]
fn test_c() {
    assert!(!Solution::is_palindrome(10));
}
