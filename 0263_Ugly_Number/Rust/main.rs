// An ugly number is a positive integer whose prime factors are limited to 2, 3, and 5.
// Given an integer n, return true if n is an ugly number.

impl Solution {
    pub fn is_ugly(n: i32) -> bool {
        if n <= 0 {
            return false;
        }
        let mut n = n;
        for i in [2, 3, 5].iter() {
            while n % i == 0 {
                n /= i;                 // El operador /= es equivalente a n = n / i
            }
        }
        n == 1                          // Si n es 1, entonces n es un numero feo
    }
}