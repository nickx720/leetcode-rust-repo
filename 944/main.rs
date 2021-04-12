impl Solution {
    pub fn min_deletion_size(strs: Vec<String>) -> i32 {
        let mut new_str: Vec<String> = vec!["".to_string(); strs[0].len()];
        for str in strs {
            for (index, item) in str.chars().enumerate() {
                new_str[index] += item.to_string().as_str();
            }
        }
        let mut col_del = 0;
        for str in new_str {
            let str = str.chars().collect::<Vec<char>>();
            for i in 1..str.len() {
                if str[i - 1] > str[i] {
                    col_del += 1;
                    break;
                }
            }
        }
        col_del
    }
}
