// Given an integer columnNumber, return its corresponding column title as it appears in an Excel sheet.
// For example: 
//      A -> 1
//      B -> 2
//      C -> 3
//      ...
//      Z -> 26
//      AA -> 27
//      AB -> 28 
//      ...

impl Solution {
    pub fn convert_to_title(column_number: i32) -> String {
        let mut column_number = column_number;
        let mut result = String::new();
        while column_number > 0 {
            column_number -= 1;
            let c = (column_number % 26) as u8 + 'A' as u8;
            result.push(c as char);
            column_number /= 26;
        }
        result.chars().rev().collect()
    }
}