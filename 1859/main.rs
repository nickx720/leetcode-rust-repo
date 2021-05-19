impl Solution {
    pub fn sort_sentence(s: String) -> String {
        let mut collection_of_s = s
        .trim()
        .split(" ")
        .collect::<Vec<&str>>()
        .iter()
        .map(|&item| {
            let new_string = &item[..item.len() - 1];
            let position = item[item.len() - 1..].parse::<usize>().unwrap();
            (position, new_string)
        })
        .collect::<Vec<(usize, &str)>>();
    collection_of_s.sort_by(|(a, _), (b, _)| a.cmp(&b));
    collection_of_s
        .iter()
        .map(|(_, a)| *a)
        .collect::<Vec<&str>>()
        .join(" ") 
    }
}