// Write an algorithm to determine if a number n is happy.
// A happy number is a number defined by the following process:
// - Starting with any positive integer, replace the number by the sum of the squares of its digits.
// - Repeat the process until the number equals 1 (where it will stay), or it loops endlessly in a cycle which does not include 1.
// - Those numbers for which this process ends in 1 are happy.
// - Return true if n is a happy number, and false if not.

impl Solution {
    pub fn is_happy(n: i32) -> bool {
        let mut n = n;
        let mut set = std::collections::HashSet::new();
        while n != 1 && !set.contains(&n) { // Mientras n sea diferente de 1 y no este en el set
            set.insert(n);
            n = n.to_string().chars().map(|c| c.to_digit(10).unwrap().pow(2)).sum::<u32>() as i32; // Separa los numeros y los eleva al cuadrado para sumarlos
        }
        n == 1
    }
}