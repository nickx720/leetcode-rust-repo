/// https://dev.to/seanpgallivan/solution-russian-doll-envelopes-459i
impl Solution{
    fn max_envelopes(envelopes: Vec<Vec<i32>>) -> i32 {
        let mut envelopes = envelopes;
        envelopes.sort_by(|a,b| {
            if a[0] == b[0] {
                b[1].cmp(&a[1])
            } else {
                a[0].cmp(&b[0])
            }
        });
        let mut dp:Vec<i32> = Vec::new();
        for item in envelopes{
            let height = item[1];
            let search_match = dp.binary_search(&height);
            let left = match search_match{
                Ok(value) => value,
                Err(value) => value,
            };
            if left == dp.len(){
                dp.push(height);
            }
            dp[left] = height;
        }
        dp.len() as i32
    }

}
