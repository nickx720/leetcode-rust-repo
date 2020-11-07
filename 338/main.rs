impl Solution{
    fn count_bits(num: i32) -> Vec<i32> {
        let input: Vec<i32> = (0..=num).collect();
        let result: Vec<i32> = input
            .into_iter()
            .map(|x| {
                let y = format!("{:b}", x);
                let sumout: u32 = y.chars().into_iter().map(|x| x.to_digit(10).unwrap()).sum();
                sumout as i32
            })
            .collect();  
        result
    }
    // alternative solution use right shift operator on res[i >>1] for prefix value which will give you value of one and for the remaining in (i as i32 & 1) 
    //we are doing a bitwise AND which gives 1 if both the last digits are one and 0 if one of them is zero
     pub fn count_bits(num: i32) -> Vec<i32> {
            let num = num as usize;
            let mut res: Vec<i32> = vec![0; num + 1];
            for i in 1..num + 1 {
                res[i] = res[i >> 1] + (i as i32 & 1);
            }
            res
        }
}