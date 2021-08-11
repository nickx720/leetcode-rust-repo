impl Solution {
    pub fn count_triples(n: i32) -> i32 {
        let mut count = 0;
        for item in 1..=n {
            for jitem in item+1..=n{
                let (item,jitem) = (item as f32, jitem as f32);
                let output = item.powi(2) + jitem.powi(2);
                let square_root = output.sqrt().trunc();
                if square_root.powi(2) == output && square_root <= n as f32{
                    count+=2;
                }
            }
        }
        count

    }
}
