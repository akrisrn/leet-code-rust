use crate::Solution;

/// https://leetcode.com/problems/longest-common-prefix/
///
/// Write a function to find the longest common prefix string amongst an array of strings.
/// If there is no common prefix, return an empty string `""`.
///
/// # Note
///
/// All given inputs are in lowercase letters `a-z`.
///
/// # Example
///
/// > Input: ["flower", "flow", "flight"]
/// > Output: "fl"
///
/// > Input: ["dog", "racecar", "car"]
/// > Output: ""
/// > Explanation: There is no common prefix among the input strings.
///
impl Solution {
    pub fn longest_common_prefix(strs: Vec<String>) -> String {
        if strs.len() < 2 {
            return "".to_string();
        }
        let mut prefix_chars: Vec<u8> = strs[0].bytes().collect();
        for str in &strs[1..] {
            for (i, ch) in str.bytes().enumerate() {
                if i < prefix_chars.len() {
                    if prefix_chars[i] != ch {
                        prefix_chars = prefix_chars[..i].to_owned();
                        if prefix_chars.is_empty() {
                            return "".to_string();
                        }
                    }
                } else {
                    break;
                }
            }
        }
        String::from_utf8(prefix_chars).unwrap()
    }
}

#[test]
fn test_a() {
    assert_eq!(Solution::longest_common_prefix(vec!["flower".to_string(), "flow".to_string(), "flight".to_string()]), "fl");
}

#[test]
fn test_b() {
    assert_eq!(Solution::longest_common_prefix(vec!["dog".to_string(), "racecar".to_string(), "car".to_string()]), "");
}

#[test]
fn test_c() {
    assert_eq!(Solution::longest_common_prefix(vec![]), "");
}
