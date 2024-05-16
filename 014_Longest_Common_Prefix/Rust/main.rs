impl Solution {
    pub fn longest_common_prefix(strs: Vec<String>) -> String {
        if strs.len() == 0 {
            return String::from("");
        }
        let mut prefix = strs[0].clone();
        for i in 1..strs.len() {
            while !strs[i].starts_with(&prefix) {
                prefix.pop();
                if prefix.is_empty() {
                    return String::from("");
                }
            }
        }
        return prefix;
    }
}
