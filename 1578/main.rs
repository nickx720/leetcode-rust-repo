impl Solution {
    pub fn min_cost(s: String, cost: Vec<i32>) -> i32 {
        let mut gmax = 0;
        let mut ans = 0;
        let mut gsum = 0;
        let s = s.chars().collect::<Vec<char>>();
        for i in 0..s.len() {
            if i > 0 && s[i] != s[i - 1] {
                ans += gsum - gmax;
                gsum = 0;
                gmax = 0;
            }
            gsum += cost[i];
            gmax = std::cmp::max(gmax, cost[i])
        }
        ans += gsum - gmax;
        ans
    }
}
