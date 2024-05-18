// Given two strings s and t, return true if t is an anagram of s, and false otherwise.
// An Anagram is a word or phrase formed by rearranging the letters of a different word or phrase, typically using all the original letters exactly once.

impl Solution {
    pub fn is_anagram(s: String, t: String) -> bool {
        let mut s = s.chars().collect::<Vec<char>>();
        let mut t = t.chars().collect::<Vec<char>>();
        s.sort();
        t.sort();
        s == t
    }
}

impl Solution {
    pub fn is_anagram(s: String, t: String) -> bool {
        let mut sh = std::collections::HashMap::new();
        let mut th = std::collections::HashMap::new();
        for c in s.chars() {
            *sh.entry(c).or_insert(0) += 1;
        }
        for c in t.chars() {
            *th.entry(c).or_insert(0) += 1;
        }
        sh == th
    }
}