impl Solution {
    pub fn num_times_all_blue(light: Vec<i32>) -> i32 {
        let mut res = 0;
    let mut cur = 1;
    let mut max_light = 0;
    let mut left: std::collections::HashMap<i32, bool> = std::collections::HashMap::new();
    for &i in light.iter() {
        left.insert(i, true);
        max_light = std::cmp::max(max_light, i);
        if i >= cur {
            while left.get(&cur) != None {
                cur += 1;
            }
            if cur - 1 == max_light {
                res += 1;
            }
        }
    }
    res
    }
}