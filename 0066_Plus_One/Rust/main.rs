impl Solution {
    pub fn plus_one(digits: Vec<i32>) -> Vec<i32> {
        let mut digits = digits;
        let mut i = digits.len() as i32 - 1;
        
        while i >= 0 {
            if digits[i as usize] < 9 {
                digits[i as usize] += 1;
                return digits;
            }
            digits[i as usize] = 0;
            i -= 1;
        }
        // El primer digito al no poder ser 0 ya que significa que el ultimo numero era 9 se añade un 1 de la llevada
        let mut result = vec![0; digits.len() + 1];
        result[0] = 1;
        result
    }
}
