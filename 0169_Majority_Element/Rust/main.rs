// Given an array nums of size n, return the majority element.
// The majority element is the element that appears more than ⌊n / 2⌋ times. You may assume that the majority element always exists in the array.

impl Solution {
    pub fn majority_element(nums: Vec<i32>) -> i32 {
        //Boyer-Moore Voting Algorithm
        let mut result = 0;
        let mut count = 0;
        for num in nums {
            if count == 0 {
                result = num;
            }
            count += if num == result { 1 } else { -1 };
        }
        result        
    }
}