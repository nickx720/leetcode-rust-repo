impl Solution {
    pub fn predict_party_victory(senate: String) -> String {
         let mut queue1: std::collections::VecDeque<usize> = std::collections::VecDeque::new();
    let mut queue2: std::collections::VecDeque<usize> = std::collections::VecDeque::new();
    let n = senate.len();
    for (index, item) in senate.chars().enumerate() {
        if item == 'R' {
            queue1.push_back(index);
        } else {
            queue2.push_back(index);
        }
    }
    while !queue1.is_empty() && !queue2.is_empty() {
        if let Some(r_index) = queue1.pop_front() {
            if let Some(d_index) = queue2.pop_front() {
                if r_index < d_index {
                    queue1.push_back(r_index + n);
                } else {
                    queue2.push_back(d_index + n);
                }
            }
        }
    }
    if queue1.len() > queue2.len() {
        return String::from("Radiant");
    } else {
        return String::from("Dire");
    }
    }
}