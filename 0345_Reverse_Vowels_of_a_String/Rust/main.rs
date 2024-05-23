// Given a string s, reverse only all the vowels in the string and return it.
// The vowels are 'a', 'e', 'i', 'o', and 'u', and they can appear in both lower and upper cases, more than once.

impl Solution {
    pub fn reverse_vowels(s: String) -> String {
        let mut s = s.chars().collect::<Vec<char>>();
        let mut i = 0;
        let mut j = s.len() - 1;
        while i < j {
            if "aeiouAEIOU".contains(s[i]) && "aeiouAEIOU".contains(s[j]) {
                s.swap(i, j);
                i += 1;
                j -= 1;
            } else if "aeiouAEIOU".contains(s[i]) {
                j -= 1;
            } else if "aeiouAEIOU".contains(s[j]) {
                i += 1;
            } else {
                i += 1;
                j -= 1;
            }
        }
        s.iter().collect()
    }
}