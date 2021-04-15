impl Solution {
    pub fn sort_by_bits(arr: Vec<i32>) -> Vec<i32> {
        let mut output = arr
            .iter()
            .map(|&x| {
                let formatted_string = format!("{:b}", x)
                    .chars()
                    .map(|y| y.to_digit(10).unwrap())
                    .sum::<u32>();
                (x, formatted_string)
            })
            .collect::<Vec<_>>();
        output.sort_by(|(c, a), (d, b)| if a != b { a.cmp(b) } else { c.cmp(d) });
        output.iter().map(|&(x, _)| x).collect()
    }
}
