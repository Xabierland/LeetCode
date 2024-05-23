// Given two integer arrays nums1 and nums2, return an array of their intersection. Each element in the result must appear as many times as it shows in both arrays and you may return the result in any order.

use std::collections::HashMap;

impl Solution {
    pub fn intersect(nums1: Vec<i32>, nums2: Vec<i32>) -> Vec<i32> {
        let mut counts = HashMap::new();
        // Contar las frecuencias de cada número en nums1
        for num in nums1 {
            *counts.entry(num).or_insert(0) += 1;
        }
        
        let mut result = Vec::new();
        // Verificar los elementos en nums2 contra el mapa de conteos
        for num in nums2 {
            if let Some(count) = counts.get_mut(&num) {
                if *count > 0 {
                    result.push(*num);
                    *count -= 1;
                }
            }
        }
        
        result
    }
}
