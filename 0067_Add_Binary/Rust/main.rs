impl Solution {
    pub fn add_binary(a: String, b: String) -> String {
        let mut a = a.chars().rev().collect::<Vec<char>>();
        let mut b = b.chars().rev().collect::<Vec<char>>();
        let mut result = String::new();
        let mut carry = 0;
        let mut i = 0;
        
        while i < a.len() || i < b.len() || carry > 0 {
            let mut sum = carry;
            if i < a.len() {
                sum += a[i].to_digit(10).unwrap();
            }
            if i < b.len() {
                sum += b[i].to_digit(10).unwrap();
            }
            carry = sum / 2;
            result.push_str(&(sum % 2).to_string());
            i += 1;
        }
        result.chars().rev().collect()
    }
}