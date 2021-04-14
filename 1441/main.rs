impl Solution {
    pub fn build_array(target: Vec<i32>, n: i32) -> Vec<String> {
        let mut output: Vec<String> = Vec::new();
        for i in 0..target[target.len() - 1] {
            output.push(String::from("Push"));
            if target.contains(&(i + 1)) {
                continue;
            };
            output.push(String::from("Pop"));
        }
        output
    }
}
