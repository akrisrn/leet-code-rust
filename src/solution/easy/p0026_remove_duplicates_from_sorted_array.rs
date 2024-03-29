use crate::Solution;

/// https://leetcode.com/problems/remove-duplicates-from-sorted-array/
///
/// Given a sorted array nums, remove the duplicates [in-place](https://en.wikipedia.org/wiki/In-place_algorithm)
/// such that each element appear only once and return the new length.
/// Do not allocate extra space for another array, you must do this by **modifying the input array**
/// [in-place](https://en.wikipedia.org/wiki/In-place_algorithm) with O(1) extra memory.
///
/// # Clarification
///
/// Confused why the returned value is an integer but your answer is an array?
/// Note that the input array is passed in by **reference**,
/// which means modification to the input array will be known to the caller as well.
/// Internally you can think of this:
///
/// > // nums is passed in by reference. (i.e., without making a copy)
/// > int len = removeDuplicates(nums);
/// >
/// > // any modification to nums in your function would be known by the caller.
/// > // using the length returned by your function, it prints the first len elements.
/// > for (int i = 0; i < len; i++) {
/// >     print(nums[i]);
/// > }
///
/// # Example
///
/// > Given nums = [1, 1, 2],
/// > Your function should return length = 2, with the first two elements of nums being 1 and 2 respectively.
/// > It doesn't matter what you leave beyond the returned length.
///
/// > Given nums = [0, 0, 1, 1, 1, 2, 2, 3, 3, 4],
/// > Your function should return length = 5,
/// > with the first five elements of nums being modified to 0, 1, 2, 3, and 4 respectively.
/// > It doesn't matter what values are set beyond the returned length.
///
impl Solution {
    pub fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {
        let mut some_pre = None;
        let mut i = 0;
        while i < nums.len() {
            if let Some(pre) = some_pre {
                if pre == nums[i] {
                    nums.remove(i);
                    i -= 1;
                }
            }
            some_pre = Some(nums[i]);
            i += 1;
        }
        nums.len() as i32
    }

    // fxxk
    pub fn remove_duplicates_0ms(nums: &mut Vec<i32>) -> i32 {
        nums.dedup();
        nums.len() as i32
    }
}

#[test]
fn test_a() {
    let mut nums = vec![1, 1, 2];
    assert_eq!(Solution::remove_duplicates(&mut nums), 2);
    assert_eq!(nums, vec![1, 2])
}

#[test]
fn test_b() {
    let mut nums = vec![0, 0, 1, 1, 1, 2, 2, 3, 3, 4];
    assert_eq!(Solution::remove_duplicates(&mut nums), 5);
    assert_eq!(nums, vec![0, 1, 2, 3, 4])
}
