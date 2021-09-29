impl Solution {
    pub fn max_number_of_balloons(text: String) -> i32 {
     use std::collections::HashMap;
    let seed_string = "balloon".to_string();
    let mut frequency_counter : HashMap<char,i32> = HashMap::new();
    seed_string.chars().for_each(|x| {
        frequency_counter.entry(x).or_default();
    });
    for item in text.chars(){
        if frequency_counter.contains_key(&item){
            *frequency_counter.entry(item).or_default()+=1;
        }
    }
    frequency_counter
        .iter()
        .map(|(&c,&num)| num / if "lo".contains(c) {2} else {1})
        .min()
        .unwrap()

    }
}
