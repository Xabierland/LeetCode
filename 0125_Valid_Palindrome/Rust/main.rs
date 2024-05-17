// A phrase is a palindrome if, after converting all uppercase letters into lowercase letters and removing all non-alphanumeric characters, it reads the same forward and backward. Alphanumeric characters include letters and numbers.
// Given a string s, return true if it is a palindrome, or false otherwise.

impl Solution {
    pub fn is_palindrome(s: String) -> bool {
        let s = s.chars().filter(|c| c.is_alphanumeric()).map(|c| c.to_ascii_lowercase()).collect::<String>();
        let mut left = 0;
        let mut right = s.len() - 1;
        if s.len() == 0 {
            return true;
        }
        while left < right {
            if let (Some(left_char), Some(right_char)) = (s.chars().nth(left), s.chars().nth(right)) { // Esto es para evitar que se caiga si left_char o right_char no tienen valor
                if left_char != right_char {
                    return false;
                }
            } else {
                return false;
            }
            left += 1;
            right -= 1;
        }
        return true
    }
}