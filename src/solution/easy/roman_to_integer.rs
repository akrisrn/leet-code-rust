use crate::Solution;

/// https://leetcode.com/problems/roman-to-integer/
///
/// Roman numerals are represented by seven different symbols: `I`, `V`, `X`, `L`, `C`, `D` and `M`.
///
/// > Symbol       Value
/// > I            1
/// > V            5
/// > X            10
/// > L            50
/// > C            100
/// > D            500
/// > M            1000
///
/// For example, two is written as `II` in Roman numeral, just two one's added together.
/// Twelve is written as, `XII`, which is simply `X` + `II`.
/// The number twenty seven is written as `XXVII`, which is `XX` + `V` + `II`.
///
/// Roman numerals are usually written largest to smallest from left to right.
/// However, the numeral for four is not `IIII`. Instead, the number four is written as `IV`.
/// Because the one is before the five we subtract it making four.
/// The same principle applies to the number nine, which is written as `IX`.
/// There are six instances where subtraction is used:
///
/// - `I` can be placed before `V` (5) and `X` (10) to make 4 and 9.
/// - `X` can be placed before `L` (50) and `C` (100) to make 40 and 90.
/// - `C` can be placed before `D` (500) and `M` (1000) to make 400 and 900.
///
/// Given a roman numeral, convert it to an integer. Input is guaranteed to be within the range from 1 to 3999.
///
/// # Example
///
/// > Input: "III"
/// > Output: 3
///
/// > Input: "IV"
/// > Output: 4
///
/// > Input: "IX"
/// > Output: 9
///
/// > Input: "LVIII"
/// > Output: 58
/// > Explanation: L = 50, V= 5, III = 3.
///
/// > Input: "MCMXCIV"
/// > Output: 1994
/// > Explanation: M = 1000, CM = 900, XC = 90 and IV = 4.
///
impl Solution {
    pub fn roman_to_int(s: String) -> i32 {
        let mut sum = 0;
        let mut next_sub = false;
        for ch in s.chars() {
            match ch {
                'I' => {
                    sum += 1;
                    next_sub = true
                }
                'V' => {
                    sum += 5;
                    if next_sub {
                        sum -= 2;
                        next_sub = false
                    }
                }
                'X' => {
                    sum += 10;
                    if next_sub {
                        sum -= 2;
                        next_sub = false
                    } else {
                        next_sub = true
                    }
                }
                'L' => {
                    sum += 50;
                    if next_sub {
                        sum -= 20;
                        next_sub = false
                    }
                }
                'C' => {
                    sum += 100;
                    if next_sub {
                        sum -= 20;
                        next_sub = false
                    } else {
                        next_sub = true
                    }
                }
                'D' => {
                    sum += 500;
                    if next_sub {
                        sum -= 200;
                        next_sub = false
                    }
                }
                'M' => {
                    sum += 1000;
                    if next_sub {
                        sum -= 200;
                        next_sub = false
                    }
                }
                _ => {}
            }
        }
        sum
    }
}

#[test]
fn test_a() {
    assert_eq!(Solution::roman_to_int("III".to_string()), 3);
}

#[test]
fn test_b() {
    assert_eq!(Solution::roman_to_int("IV".to_string()), 4);
}

#[test]
fn test_c() {
    assert_eq!(Solution::roman_to_int("IX".to_string()), 9);
}

#[test]
fn test_d() {
    assert_eq!(Solution::roman_to_int("LVIII".to_string()), 58);
}

#[test]
fn test_e() {
    assert_eq!(Solution::roman_to_int("MCMXCIV".to_string()), 1994);
}
