impl Solution {
    pub fn num_special_equiv_groups(a: Vec<String>) -> i32 {
        use std::collections::HashSet;
        let mut output: HashSet<String> = HashSet::new();
        for item in a {
            // Collect odd occurences of item using iter
            let mut odd: Vec<char> = item.chars().step_by(2).collect();
            // Collect even occureces of char in item
            let mut even: Vec<char> = item.chars().skip(1).step_by(2).collect();
            odd.sort();
            even.sort();
            let key = odd.iter().collect::<String>() + &even.iter().collect::<String>();
            output.insert(key);
        }
        output.len() as i32
    }
}
