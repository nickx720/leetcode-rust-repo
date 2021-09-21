/// https://dev.to/seanpgallivan/solution-reconstruct-original-digits-from-english-4o2p
impl Solution {
    pub fn original_digits(s: String) -> String {
        let len = (26 + 'a' as u32) as usize;
        let mut item:Vec<u32> = vec![0;len];
        for c in s.chars(){
            item[c as usize] +=1;
        }
        let mut res = vec![0;10];
        res[0] = item['z' as usize];
        res[2] = item['w' as usize];
        res[4] = item['u' as usize];
        res[6] = item['x' as usize];
        res[8] = item['g' as usize];
        res[3] = item['h' as usize] - res[8];
        res[5] = item['f' as usize] - res[4];
        res[7] = item['s' as usize] - res[6];
        res[9] = item['i' as usize] - res[5] - res[6] - res[8];
        res[1] = item['o' as usize] - res[0] - res[2] - res[4];

        let mut result = String::new();
        for i in 0..10{
            result.push_str(&i.to_string().repeat(res[i] as usize));
        }
        result

    }
}
