// Given a pattern and a string s, find if s follows the same pattern.
// Here follow means a full match, such that there is a bijection between a letter in pattern and a non-empty word in s.

impl Solution {
    pub fn word_pattern(pattern: String, s: String) -> bool {
        let mut pattern_map = std::collections::HashMap::new();
        let mut s_map = std::collections::HashMap::new();
        let pattern = pattern.chars().collect::<Vec<char>>();
        let s = s.split_whitespace().collect::<Vec<&str>>();
        if pattern.len() != s.len() {                                               // Si la longitud de los dos vectores es diferente, no puede haber una correspondencia
            return false;
        }
        for i in 0..pattern.len() {                                                 // Iteramos sobre los dos vectores
            if pattern_map.insert(pattern[i], i) != s_map.insert(s[i], i) {         // Si la insercion falla, significa que ya existe una correspondencia
                return false;
            }
        }
        true
    }
}