use crate::Solution;

/// https://leetcode.com/problems/implement-strstr/
///
/// Implement [strStr()](http://www.cplusplus.com/reference/cstring/strstr/).
/// Return the index of the first occurrence of needle in haystack, or **-1** if needle is not part of haystack.
///
/// # Clarification
///
/// What should we return when `needle` is an empty string? This is a great question to ask during an interview.
/// For the purpose of this problem, we will return 0 when `needle` is an empty string.
/// This is consistent to C's [strstr()](http://www.cplusplus.com/reference/cstring/strstr/)
/// and Java's [indexOf()](https://docs.oracle.com/javase/7/docs/api/java/lang/String.html#indexOf(java.lang.String)).
///
/// # Example
///
/// > Input: haystack = "hello", needle = "ll"
/// > Output: 2
///
/// > Input: haystack = "aaaaa", needle = "bba"
/// > Output: -1
///
impl Solution {
    pub fn str_str(haystack: String, needle: String) -> i32 {
        if needle.is_empty() {
            return 0;
        }
        match haystack.find(&needle) {
            Some(i) => i as i32,
            None => -1,
        }
    }

    pub fn str_str_another(haystack: String, needle: String) -> i32 {
        if needle.is_empty() {
            return 0;
        }
        if haystack.len() < needle.len() {
            return -1;
        }
        // easy solution
        // haystack.find(&needle).map_or(-1_i32, |v| v as i32)

        // naive solution
        let haystack_chars: Vec<char> = haystack.chars().collect();
        let needle_chars: Vec<char> = needle.chars().collect();
        let mut curr_idx = 0;
        let stop_pos = haystack_chars.len() - needle_chars.len() + 1;
        loop {
            let mut i = curr_idx;
            let mut j = 0;
            while haystack_chars[i] == needle_chars[j] {
                i += 1;
                j += 1;
                if j == needle_chars.len() {
                    return curr_idx as i32;
                }
            }
            curr_idx += 1;
            if curr_idx >= stop_pos {
                break;
            }
        }
        return -1;
    }
}

#[test]
fn test_a() {
    assert_eq!(Solution::str_str("hello".to_string(), "ll".to_string()), 2)
}

#[test]
fn test_b() {
    assert_eq!(
        Solution::str_str("aaaaa".to_string(), "bba".to_string()),
        -1
    )
}

#[test]
fn test_c() {
    assert_eq!(Solution::str_str("hello".to_string(), "".to_string()), 0)
}

#[test]
fn test_d() {
    assert_eq!(Solution::str_str("aaa".to_string(), "aaaa".to_string()), -1)
}

#[test]
fn test_e() {
    assert_eq!(
        Solution::str_str("mississippi".to_string(), "issip".to_string()),
        4
    )
}
