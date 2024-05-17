// Given an integer array nums and an integer k, return true if there are two distinct indices i and j in the array such that nums[i] == nums[j] and abs(i - j) <= k.

impl Solution {
    pub fn contains_nearby_duplicate(nums: Vec<i32>, k: i32) -> bool {
        use std::collections::HashMap;
        let mut map = HashMap::new();
        for (i, num) in nums.iter().enumerate() {
            if let Some(j) = map.get(num) {
                if (i - j) as i32 <= k {
                    return true;
                }
            }
            map.insert(num, i);
        }
        false
    }
}