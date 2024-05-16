impl Solution {
    pub fn get_row(row_index: i32) -> Vec<i32> {
        let mut res: Vec<i32> = vec![1; row_index as usize + 1];
        for i in 1..row_index as usize {
            for j in (1..=i).rev() {
                res[j] += res[j - 1];
            }
        }
        res       
    }
}