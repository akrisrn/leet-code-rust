use crate::Solution;
use std::collections::HashMap;

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
        let mut pre_ch = ' ';
        for ch in s.chars() {
            match ch {
                'I' => {
                    sum += 1;
                    pre_ch = 'I'
                }
                'V' => {
                    sum += 5;
                    if pre_ch == 'I' {
                        sum -= 2;
                    }
                    pre_ch = ' '
                }
                'X' => {
                    sum += 10;
                    if pre_ch == 'I' {
                        sum -= 2;
                        pre_ch = ' '
                    } else {
                        pre_ch = 'X'
                    }
                }
                'L' => {
                    sum += 50;
                    if pre_ch == 'X' {
                        sum -= 20;
                    }
                    pre_ch = ' '
                }
                'C' => {
                    sum += 100;
                    if pre_ch == 'X' {
                        sum -= 20;
                        pre_ch = ' '
                    } else {
                        pre_ch = 'C'
                    }
                }
                'D' => {
                    sum += 500;
                    if pre_ch == 'C' {
                        sum -= 200;
                    }
                    pre_ch = ' '
                }
                'M' => {
                    sum += 1000;
                    if pre_ch == 'C' {
                        sum -= 200;
                    }
                    pre_ch = ' '
                }
                _ => {}
            }
        }
        sum
    }

    pub fn roman_to_int_0ms(s: String) -> i32 {
        let map: HashMap<char, i32> = vec![
            ('I', 1),
            ('V', 5),
            ('X', 10),
            ('L', 50),
            ('C', 100),
            ('D', 500),
            ('M', 1000),
        ].into_iter().collect();
        let mut last = 10000;
        let mut result = 0;
        s.chars().for_each(|c| {
            let current = *map.get(&c).unwrap();
            if current <= last {
                result += current;
            } else {
                result += current - 2 * last;
            }
            last = current;
        });
        result
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

#[test]
fn test_f() {
    assert_eq!(Solution::roman_to_int("DCXXI".to_string()), 621);
}
