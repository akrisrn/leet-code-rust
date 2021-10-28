use crate::Solution;

/// https://leetcode.com/problems/count-and-say/
///
/// The count-and-say sequence is the sequence of integers with the first five terms as following:
///
/// > 1.     1
/// > 2.     11
/// > 3.     21
/// > 4.     1211
/// > 5.     111221
///
/// `1` is read off as `"one 1"` or `11`.
/// `11` is read off as `"two 1s"` or `21`.
/// `21` is read off as `"one 2`, then `one 1"` or `1211`.
///
/// Given an integer n where 1 ≤ n ≤ 30, generate the n^th term of the count-and-say sequence.
/// Note: Each term of the sequence of integers will be represented as a string.
///
/// # Example
///
/// > Input: 1
/// > Output: "1"
///
/// > Input: 4
/// > Output: "1211"
///
/// # Misleading description
///
/// https://leetcode.com/problems/count-and-say/discuss/16015/Please-change-the-misleading-description
///
impl Solution {
    pub fn count_and_say(n: i32) -> String {
        if n == 1 {
            return "1".to_string();
        }
        let mut result = String::new();
        let chars: Vec<char> = Solution::count_and_say(n - 1).chars().collect();
        let mut count = (chars[0], 0);
        for ch in chars {
            if ch == count.0 {
                count.1 += 1
            } else {
                result.push_str(count.1.to_string().as_ref());
                result.push_str(count.0.to_string().as_ref());
                count = (ch, 1)
            }
        }
        result.push_str(count.1.to_string().as_ref());
        result.push_str(count.0.to_string().as_ref());
        result
    }

    pub fn count_and_say_another(n: i32) -> String {
        let mut out = "1".to_string();
        for _i in 1..n {
            let mut x = vec![];
            let mut iter = out.chars().peekable();
            while let Some(c) = iter.next() {
                let mut count = 1;
                while let Some(o) = iter.peek() {
                    if c != *o {
                        break;
                    }
                    count += 1;
                    iter.next();
                }
                x.push(format!("{}{}", count, c));
            }
            out = x.join("");
        }
        out
    }
}

#[test]
fn test_a() {
    assert_eq!(Solution::count_and_say(1), "1".to_string())
}

#[test]
fn test_b() {
    assert_eq!(Solution::count_and_say(4), "1211".to_string())
}

#[test]
fn test_c() {
    assert_eq!(Solution::count_and_say(7), "13112221".to_string())
}
