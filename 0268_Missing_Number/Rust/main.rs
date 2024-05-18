// Given an array nums containing n distinct numbers in the range [0, n], return the only number in the range that is missing from the array.

impl Solution {
    pub fn missing_number(nums: Vec<i32>) -> i32 {
        let n = nums.len() as i32;
        n * (n + 1) / 2 - nums.iter().sum::<i32>()
    }
}