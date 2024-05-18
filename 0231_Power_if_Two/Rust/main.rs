// Given an integer n, return true if it is a power of two. Otherwise, return false.
// An integer n is a power of two, if there exists an integer x such that n == 2x.

impl Solution {
    pub fn is_power_of_two(n: i32) -> bool {
        n > 0 && n & (n - 1) == 0 // n > 0: n debe ser positivo, n & (n - 1) == 0: n debe tener solo un bit en 1 al hacer la operación AND
    }
}