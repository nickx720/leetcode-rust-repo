impl Solution {
    pub fn frequency_sort(nums: Vec<i32>) -> Vec<i32> {
         use std::collections::HashMap;
    let mut count: HashMap<i32, i32> = HashMap::new();
    for item in nums {
        if count.contains_key(&item) {
            if let Some(counter) = count.get_mut(&item) {
                *counter += 1;
            };
        } else {
            count.insert(item, 1);
        }
    }
    let mut count = count.into_iter().collect::<Vec<(i32,i32)>>();
    count.sort_by(|(c,a),(d,b)| if a == b {d.cmp(c)} else {a.cmp(b)});
    count.iter().map(|&(a,b)| vec![a;b as usize]).flatten().collect()
    }
}