impl Solution {
    pub fn beautiful_array(n: i32) -> Vec<i32> {
        let mut output: Vec<i32> = vec![1];
        while output.len() < n as usize {
            let mut temp: Vec<i32> = Vec::new();
            for i in &output {
                if i * 2 - 1 <= n {
                    //    Push odd numbers into output
                    temp.push(i * 2 - 1);
                }
            }
            for i in &output {
                if i * 2 <= n {
                    //    push even numbers
                    temp.push(i * 2);
                }
            }
            output = temp;
        }
        output
    }
}
