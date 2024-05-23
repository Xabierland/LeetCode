// Given an integer n, return true if it is a power of three. Otherwise, return false.
// An integer n is a power of three, if there exists an integer x such that n == 3x.

impl Solution {
    pub fn is_power_of_three(n: i32) -> bool {
        n > 0 && 1162261467 % n == 0 // 1162261467 es el mayor número que es potencia de 3 y que cabe en un i32
    }
}

impl Solution2 {
    pub fn is_power_of_three(n: i32) -> bool {
        if n <= 0 {
            return false;
        }     
        let mut n = n;
        while n/3!=0 {
            if n %3 != 0{
                return false;
            }
            n = n/3;
        }
        return n==1

    }
}