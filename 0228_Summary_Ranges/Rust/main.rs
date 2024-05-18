// You are given a sorted unique integer array nums.
// A range [a,b] is the set of all integers from a to b (inclusive).
// Return the smallest sorted list of ranges that cover all the numbers in the array exactly. That is, each element of nums is covered by exactly one of the ranges, and there is no integer x such that x is in one of the ranges but not in nums.
// Each range [a,b] in the list should be output as:
//     "a->b" if a != b
//     "a" if a == b

impl Solution {
    pub fn summary_ranges(nums: Vec<i32>) -> Vec<String> {
        let mut res = Vec::new();
        let mut i = 0;
        while i < nums.len() {                                                          // i se usa para recorrer el vector
            let mut j = i + 1;
            while j < nums.len() && nums[j] == nums[j - 1] + 1 {                        // j se usa indicar el numero de elementos que se encuentran en el rango
                j += 1;
            }
            if j - i == 1 {                                                             // si solo hay un elemento en el rango
                res.push(nums[i].to_string());
            } else {                                                                    // si hay mas de un elemento en el rango
                res.push(format!("{}->{}", nums[i], nums[j - 1]));
            }
            i = j;
        }
        res
    }
}