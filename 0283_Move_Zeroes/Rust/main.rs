// Given an integer array nums, move all 0's to the end of it while maintaining the relative order of the non-zero elements.
// Note that you must do this in-place without making a copy of the array.

impl Solution {
    pub fn move_zeroes(nums: &mut Vec<i32>) {
        let mut i = 0;
        for j in 0..nums.len() {
            if nums[j] != 0 {
                nums.swap(i, j);
                i += 1;
            }
        }
    }
}