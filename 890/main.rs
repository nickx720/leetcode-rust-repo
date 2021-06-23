impl Solution {
    fn find_and_replace_pattern(words: Vec<String>, pattern: String) -> Vec<String> {
        let mut output:Vec<String> = Vec::new();
        for word in words{
            if  match_string(word.bytes().collect(), pattern.bytes().collect()) {
                output.push(word);
            }
        }
        output
    }

    fn match_string(word:Vec<u8>,pattern:Vec<u8>)->bool {
        use std::collections::HashMap;
        let mut map_one:HashMap<u8,u8> = HashMap::new();
        let mut map_two:HashMap<u8,u8> = HashMap::new();
        for i in 0..word.len(){
            let w = word[i];
            let p = pattern[i];
            if !map_one.contains_key(&w) {
                map_one.insert(w, p);
            }
            if !map_two.contains_key(&p){
                map_two.insert(p, w);
            }
            if *map_one.get(&w).unwrap() != p || *map_two.get(&p).unwrap() != w {
                return false;
            }

        }
        true
    }



}
