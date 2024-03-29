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
        if strs.is_empty() {
            return "".to_string();
        }
        if strs.len() == 1 {
            return strs[0].to_string();
        }
        let mut prefix_chars: Vec<u8> = strs[0].bytes().collect();
        for str in &strs[1..] {
            let bytes = str.bytes();
            let len = bytes.len();
            for (i, ch) in bytes.enumerate() {
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
            if len < prefix_chars.len() {
                prefix_chars = prefix_chars[..len].to_owned()
            }
        }
        String::from_utf8(prefix_chars).unwrap()
    }

    pub fn longest_common_prefix_another(strs: Vec<String>) -> String {
        if strs.len() < 1 {
            return String::new();
        }
        let mut prefix = strs.get(0).unwrap().to_string();
        for i in 1..strs.len() {
            let mut match_char: Vec<char> = Vec::new();
            for (c1, c2) in prefix.chars().zip(strs[i].chars()) {
                if c1 == c2 {
                    match_char.push(c1);
                } else {
                    break;
                }
            }
            prefix = match_char.iter().collect();
            if prefix.len() == 0 {
                return String::new();
            }
        }
        return prefix;
    }
}

#[test]
fn test_a() {
    assert_eq!(
        Solution::longest_common_prefix(vec![
            "flower".to_string(),
            "flow".to_string(),
            "flight".to_string()
        ]),
        "fl"
    );
}

#[test]
fn test_b() {
    assert_eq!(
        Solution::longest_common_prefix(vec![
            "dog".to_string(),
            "racecar".to_string(),
            "car".to_string()
        ]),
        ""
    );
}

#[test]
fn test_c() {
    assert_eq!(Solution::longest_common_prefix(vec![]), "");
}

#[test]
fn test_d() {
    assert_eq!(Solution::longest_common_prefix(vec!["a".to_string()]), "a");
}

#[test]
fn test_e() {
    assert_eq!(
        Solution::longest_common_prefix(vec!["aa".to_string(), "a".to_string()]),
        "a"
    );
}
