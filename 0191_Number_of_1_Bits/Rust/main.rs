// Write a function that takes the binary representation of a positive integer and returns the number of 
// set bits it has (also known as the Hamming weight).


impl Solution {
    pub fn hamming_weight(n: i32) -> i32 {
        let mut result = 0;
        let mut n = n;
        while n != 0 {
            result += n & 1;
            n >>= 1;
        }
        result
    }
}