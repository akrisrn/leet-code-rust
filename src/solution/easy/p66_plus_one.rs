use crate::Solution;

/// https://leetcode.com/problems/plus-one/
///
/// Given a **non-empty** array of digits representing a non-negative integer, plus one to the integer.
/// The digits are stored such that the most significant digit is at the head of the list,
/// and each element in the array contain a single digit.
/// You may assume the integer does not contain any leading zero, except the number 0 itself.
///
/// # Example
///
/// > Input: [1, 2, 3]
/// > Output: [1, 2, 4]
/// > Explanation: The array represents the integer 123.
///
/// > Input: [4, 3, 2, 1]
/// > Output: [4, 3, 2, 2]
/// > Explanation: The array represents the integer 4321.
///
impl Solution {
    pub fn plus_one(digits: Vec<i32>) -> Vec<i32> {
        let mut new = digits;
        let mut i = new.len() - 1;
        new[i] += 1;
        while new[i] == 10 {
            new[i] = 0;
            if i == 0 {
                new.insert(0, 1);
                break;
            }
            i -= 1;
            new[i] += 1;
        }
        new
    }
}

#[test]
fn test_a() {
    assert_eq!(Solution::plus_one(vec![1, 2, 3]), vec![1, 2, 4])
}

#[test]
fn test_b() {
    assert_eq!(Solution::plus_one(vec![4, 3, 2, 1]), vec![4, 3, 2, 2])
}

#[test]
fn test_c() {
    assert_eq!(
        Solution::plus_one(vec![9, 8, 7, 6, 5, 4, 3, 2, 1, 0]),
        vec![9, 8, 7, 6, 5, 4, 3, 2, 1, 1]
    )
}

#[test]
fn test_d() {
    assert_eq!(
        Solution::plus_one(vec![
            7, 2, 8, 5, 0, 9, 1, 2, 9, 5, 3, 6, 6, 7, 3, 2, 8, 4, 3, 7, 9, 5, 7, 7, 4, 7, 4, 9, 4,
            7, 0, 1, 1, 1, 7, 4, 0, 0, 6
        ]),
        vec![
            7, 2, 8, 5, 0, 9, 1, 2, 9, 5, 3, 6, 6, 7, 3, 2, 8, 4, 3, 7, 9, 5, 7, 7, 4, 7, 4, 9, 4,
            7, 0, 1, 1, 1, 7, 4, 0, 0, 7
        ]
    )
}

#[test]
fn test_e() {
    assert_eq!(
        Solution::plus_one(vec![9, 9, 9, 9, 9, 9, 9, 9, 9]),
        vec![1, 0, 0, 0, 0, 0, 0, 0, 0, 0]
    )
}
