use std::collections::HashMap;

impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut num_map = HashMap::new(); // Create a hash map to store numbers and their indices
        
        for (i, &num) in nums.iter().enumerate() {
            let complement = target - num; // Calculate the complement
            if let Some(&index) = num_map.get(&complement) {
                return vec![index as i32, i as i32]; // If complement is found, return indices
            }
            num_map.insert(num, i); // Otherwise, insert the number and its index into the hash map
        }
        
        vec![] // This line should never be reached because there is always exactly one solution
    }
}
