impl Solution {
    pub fn dest_city(paths: Vec<Vec<String>>) -> String {
        let mut set: std::collections::HashSet<String> = std::collections::HashSet::new();
        for word in paths.iter() {
            set.insert(word[1].clone());
        }
        for word in paths.iter() {
            if set.contains(&word[0]) {
                set.remove(&word[0]);
            }
        }
        let output = set.into_iter().collect::<Vec<String>>();
        output[0].to_string()
    }
}
