use crate::Solution;
use std::collections::HashMap;

/// https://leetcode.com/problems/valid-parentheses/
///
/// Given a string containing just the characters `'('`, `')'`, `'{'`, `'}'`, `'['` and `']'`,
/// determine if the input string is valid.
///
/// An input string is valid if:
///
/// - Open brackets must be closed by the same type of brackets.
/// - Open brackets must be closed in the correct order.
///
/// Note that an empty string is also considered valid.
///
/// # Example
///
/// > Input: "()"
/// > Output: true
///
/// > Input: "()[]{}"
/// > Output: true
///
/// > Input: "(]"
/// > Output: false
///
/// > Input: "([)]"
/// > Output: false
///
/// > Input: "{[]}"
/// > Output: true
///
impl Solution {
    pub fn is_valid(s: String) -> bool {
        let map: HashMap<char, char> = vec![
            ('(', ')'),
            ('{', '}'),
            ('[', ']'),
        ].into_iter().collect();
        let mut stack = vec![];
        for char in s.chars() {
            match char {
                ')' | '}' | ']' => {
                    if let Some(ch) = stack.pop() {
                        if let Some(&c) = map.get(&ch) {
                            if c == char {
                                break;
                            }
                        }
                    }
                    return false;
                }
                _ => {
                    stack.push(char)
                }
            }
        }
        true
    }
}

#[test]
fn test_a() {
    assert!(Solution::is_valid("()".to_string()));
}

#[test]
fn test_b() {
    assert!(Solution::is_valid("()[]{}".to_string()));
}

#[test]
fn test_c() {
    assert!(!Solution::is_valid("(]".to_string()));
}

#[test]
fn test_d() {
    assert!(!Solution::is_valid("([)]".to_string()));
}

#[test]
fn test_e() {
    assert!(Solution::is_valid("{[]}".to_string()));
}

#[test]
fn test_f() {
    assert!(Solution::is_valid("".to_string()));
}
