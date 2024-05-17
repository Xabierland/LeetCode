// Given two strings s and t, determine if they are isomorphic.
// Two strings s and t are isomorphic if the characters in s can be replaced to get t.
// All occurrences of a character must be replaced with another character while preserving the order of characters. No two characters may map to the same character, but a character may map to itself.

impl Solution {
    pub fn is_isomorphic(s: String, t: String) -> bool {
        use std::collections::HashMap;

        let mut map_s_to_t = HashMap::new();
        let mut map_t_to_s = HashMap::new();

        for (char_s, char_t) in s.chars().zip(t.chars()) {
            if let Some(&mapped_char) = map_s_to_t.get(&char_s) {
                if mapped_char != char_t {
                    return false;
                }
            } else {
                map_s_to_t.insert(char_s, char_t);
            }

            if let Some(&mapped_char) = map_t_to_s.get(&char_t) {
                if mapped_char != char_s {
                    return false;
                }
            } else {
                map_t_to_s.insert(char_t, char_s);
            }
        }

        true
    }
}
