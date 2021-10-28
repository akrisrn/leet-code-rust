use crate::Solution;

/// https://leetcode.com/problems/climbing-stairs/
///
/// You are climbing a stair case. It takes n steps to reach to the top.
/// Each time you can either climb 1 or 2 steps. In how many distinct ways can you climb to the top?
/// Note: Given n will be a positive integer.
///
/// # Example
///
/// > Input: 2
/// > Output: 2
/// > Explanation: There are two ways to climb to the top.
/// > 1. 1 step + 1 step
/// > 2. 2 steps
///
/// > Input: 3
/// > Output: 3
/// > Explanation: There are three ways to climb to the top.
/// > 1. 1 step + 1 step + 1 step
/// > 2. 1 step + 2 steps
/// > 3. 2 steps + 1 step
///
impl Solution {
    pub fn climb_stairs(n: i32) -> i32 {
        let max_digits = n;
        let min_digits = (n as f64 / 2.0).ceil() as i32;
        let mut count = 1;
        let mut count_2 = 1;
        for i in (min_digits..max_digits).rev() {
            // c(count_2, i)
            let mut a = 1;
            for j in (i - count_2 + 1) as u128..=i as u128 {
                a *= j;
            }
            let mut b = 1;
            for j in 1..=count_2 as u128 {
                b *= j;
            }
            count += a / b;
            count_2 += 1;
        }
        count as i32
    }

    pub fn climb_stairs_another(n: i32) -> i32 {
        // Fibonacci sequence
        (0..n).fold((0, 1), |acc, _| (acc.1, acc.0 + acc.1)).1
    }
}

#[test]
fn test_a() {
    assert_eq!(Solution::climb_stairs(2), 2)
}

#[test]
fn test_b() {
    assert_eq!(Solution::climb_stairs(3), 3)
}

#[test]
fn test_c() {
    assert_eq!(Solution::climb_stairs(4), 5)
}

#[test]
fn test_d() {
    assert_eq!(Solution::climb_stairs(5), 8)
}

#[test]
fn test_e() {
    assert_eq!(Solution::climb_stairs(6), 13)
}

#[test]
fn test_f() {
    assert_eq!(Solution::climb_stairs(35), 14930352)
}

#[test]
fn test_g() {
    assert_eq!(Solution::climb_stairs(44), 1134903170)
}
