// Given two integer arrays nums1 and nums2, return an array of their intersection. Each element in the result must be unique and you may return the result in any order.

use std::collections::HashSet;

impl Solution {
    pub fn intersection(nums1: Vec<i32>, nums2: Vec<i32>) -> Vec<i32> {
        let set1: HashSet<i32> = nums1.into_iter().collect();
        let mut intersection = HashSet::new();
        for num in nums2 {
            if set1.contains(&num) {
                intersection.insert(num);
            }
        }
        intersection.into_iter().collect()
    }
}
