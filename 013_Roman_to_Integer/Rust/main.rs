use std::collections::HashMap;

impl Solution {
    pub fn roman_to_int(s: String) -> i32 
    {
        let mut roman_numerals = HashMap::new();
        roman_numerals.insert('I', 1);
        roman_numerals.insert('V', 5);
        roman_numerals.insert('X', 10);
        roman_numerals.insert('L', 50);
        roman_numerals.insert('C', 100);
        roman_numerals.insert('D', 500);
        roman_numerals.insert('M', 1000);

        let mut result = 0;
        let mut prev = 0;
        for c in s.chars().rev()
        {
            let value = roman_numerals.get(&c).unwrap();
            if *value < prev
            {
                result -= value;
            }
            else
            {
                result += value;
            }
            prev = *value;
        }
        return result
    }
}