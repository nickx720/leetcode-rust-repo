impl Solution {
    pub fn count_matches(items: Vec<Vec<String>>, rule_key: String, rule_value: String) -> i32 {
        let item_index: usize = match &rule_key[..] {
            "type" => 0,
            "color" => 1,
            "name" => 2,
            _ => unreachable!(),
        };
        let mut count = 0;
        for item in items {
            if item[item_index] == rule_value {
                count += 1;
            }
        }
        count
    }
}
