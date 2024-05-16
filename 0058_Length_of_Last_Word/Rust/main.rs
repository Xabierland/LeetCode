impl Solution {
    pub fn length_of_last_word(s: String) -> i32 {
        let mut i = 0;
        for c in s.chars().rev() {
            if c == ' ' {
                if i > 0 {
                    break;
                }
            } else {
                i += 1;
            }
        }
        return i as i32;
    }
}