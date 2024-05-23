// Given an integer n, return an array ans of length n + 1 such that for each i (0 <= i <= n), ans[i] is the number of 1's in the binary representation of i.

impl Solution {
    pub fn count_bits(n: i32) -> Vec<i32> {
        let mut ans = vec![0; n as usize + 1];
        for i in 1..=n as usize {
            ans[i] = ans[i & (i - 1)] + 1;
        }
        ans
    }
}

impl Solution2 {
    pub fn count_bits(n: i32) -> Vec<i32> {
        let n = n as usize;
        let mut out = vec![0; n + 1];
        for i in 1..=n {
            out[i] = out[i >> 1] + (i & 1) as i32;
        }
        out
    }
}