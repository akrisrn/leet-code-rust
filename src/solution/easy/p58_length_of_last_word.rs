use crate::Solution;

/// https://leetcode.com/problems/length-of-last-word/
///
/// Given a string s consists of upper/lower-case alphabets and empty space characters `' '`,
/// return the length of last word in the string.
/// If the last word does not exist, return 0.
/// Note: A word is defined as a character sequence consists of non-space characters only.
///
/// # Example
///
/// > Input: "Hello World"
/// > Output: 5
///
impl Solution {
    pub fn length_of_last_word(s: String) -> i32 {
        let a: Vec<&str> = s.split(" ").collect();
        if a.is_empty() {
            return 0;
        }
        a[a.len() - 1].len() as i32
    }
}

#[test]
fn test_a() {
    assert_eq!(Solution::length_of_last_word("Hello World".to_string()), 5)
}

#[test]
fn test_b() {
    assert_eq!(Solution::length_of_last_word("a".to_string()), 1)
}
