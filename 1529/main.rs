impl Solution {
    pub fn min_flips(target: String) -> i32 {
        let mut counter = 1;
        let target = target.chars().collect::<Vec<char>>();
        let first_one = target.iter().position(|&x| x == '1');
        if let Some(first_one) = first_one {
            let target = target[first_one..].to_owned();
            for item in 1..target.len() {
                if target[item] != target[item - 1] {
                    counter += 1;
                }
            }
            counter
        } else {
            0
        }
    }
}
