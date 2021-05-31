impl Solution {
    pub fn num_unique_emails(emails: Vec<String>) -> i32 {
        use std::collections::HashMap;
    let mut count_hash: HashMap<String, i32> = HashMap::new();
    for email in emails {
        let trimmed = email.trim().split("@").collect::<Vec<&str>>();
        let (local, domain) = (trimmed[0], trimmed[1]);
        let local = local
            .chars()
            .filter(|&x| x != '.')
            .collect::<Vec<char>>()
            .into_iter()
            .collect::<String>();
        if local.contains("+") {
            let local = *local.split("+").collect::<Vec<&str>>().first().unwrap();
            let key = local.to_string() + "@" + domain;
            if !count_hash.contains_key(&key) {
                let count = count_hash.entry(key).or_insert(0);
                *count += 1;
            }
        } else {
            let key = local + "@" + domain;
            if !count_hash.contains_key(&key) {
                let count = count_hash.entry(key).or_insert(0);
                *count += 1;
            }
        }
    }
    count_hash.len() as i32
    }
}