impl Solution {
    pub fn maximum69_number(num: i32) -> i32 {
        let num = num
            .to_string()
            .chars()
            .filter_map(|x| x.to_digit(10))
            .map(|x| x as i32)
            .collect::<Vec<i32>>();
        let mut output: Vec<i32> = Vec::new();
        let mut count = 0;
        for i in num {
            if i == 6 && count == 0 {
                output.push(9);
                count += 1;
                continue;
            }
            output.push(i);
        }
        let output = output
            .into_iter()
            .map(|x| x.to_string())
            .collect::<Vec<String>>()
            .join("")
            .parse::<i32>()
            .unwrap();
        output
    }
}
