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
        let mut num = 0_u64;
        let mut len = digits.len() as u32;
        for i in 0..len {
            num += 10_u64.pow(len - i - 1) * digits[i as usize] as u64
        }
        num += 1;
        let mut result = vec![];
        len = (num as f64).log10() as u32 + 1;
        for i in 0..len {
            result.push((num / 10_u64.pow(len - i - 1) % 10) as i32)
        }
        result
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
    assert_eq!(Solution::plus_one(vec![9, 8, 7, 6, 5, 4, 3, 2, 1, 0]), vec![9, 8, 7, 6, 5, 4, 3, 2, 1, 1])
}
