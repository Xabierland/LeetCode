// Reverse bits of a given 32 bits unsigned integer.

impl Solution {
    pub fn reverse_bits(x: u32) -> u32 {
        let mut result = 0;
        for i in 0..32 {
            result = result << 1 | (x >> i & 1);
        }
        result
    }
}