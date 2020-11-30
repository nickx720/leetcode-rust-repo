impl Solution {
    pub fn create_sorted_array(instructions: Vec<i32>) -> i32 {
        let mut cost = 0;
        let modulo = 10u32.pow(9) as i32 + 7;
        let mut ft: Vec<i32> = vec![0; (1e5 + 1.0) as usize];
        let mut cnt: Vec<i32> = vec![0; (1e5 + 1.0) as usize];
        for i in 0..instructions.len() {
            let mut num = instructions[i];
            let mut tot = 0;
            while num > 0 {
                tot += ft[num as usize];
                num -= num & (-num);
            }
            cost += std::cmp::min(tot, i as i32 - tot - cnt[instructions[i] as usize]);
            cost %= modulo;
            cnt[instructions[i] as usize] += 1;
            num = instructions[i] + 1;
            while num <= 1e5 as i32 {
                ft[num as usize] += 1;
                num += num & (-num);
            }
        }
        cost
    }
}
