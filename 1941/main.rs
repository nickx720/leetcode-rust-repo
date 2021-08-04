impl Solution { 
    // 1941
    fn are_occurrences_equal(s: String) -> bool {
        use std::collections::HashMap;
        let mut count_of_set:HashMap<char,i32> = HashMap::new();
        for character in s.chars(){
            *count_of_set.entry(character).or_insert(0)+=1;
        }
        if count_of_set.values().min() == count_of_set.values().max() {
            true
        } else {
            false
        }
    }

}
