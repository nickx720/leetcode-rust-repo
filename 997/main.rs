impl Solution {
    pub fn find_judge(n: i32, trust: Vec<Vec<i32>>) -> i32 {
        let mut count: Vec<i32> = vec![0; (n + 1) as usize];
        for t in trust.iter() {
            let (a, b) = (t[0], t[1]);
            count[a as usize] -= 1;
            count[b as usize] += 1;
        }
        for i in 1..=n as usize {
            if count[i] == n - 1 {
                return i as i32;
            }
        }
        -1
    }
}
