// https://dev.to/seanpgallivan/solution-vowel-spellchecker-22o
impl Solution {
    pub fn spellchecker(wordlist: Vec<String>, queries: Vec<String>) -> Vec<String> {
            use std::collections::{HashSet,HashMap};
    let mut any_matches = HashSet::new();
    let mut capitalization = HashMap::new();
    let mut vowels = HashMap::new();
    let replace_vowels = |s: &String| -> String{
        s.to_ascii_lowercase().replace(|c| match c {
            'a' | 'e' | 'i' | 'o' | 'u' => true,
            _=> false,
        }, "*")
    };

    for word in wordlist.iter(){
        any_matches.insert(word);
        capitalization.entry(word.to_ascii_lowercase()).or_insert(word);
        vowels.entry(replace_vowels(word)).or_insert(word);
    }
    let conditional_check = |s :&String| -> String{
        if let Some(&s) = any_matches.get(&s){
            return s.clone();
        };
        if let Some(&collect) = capitalization.get(&s.to_ascii_lowercase()){
            return collect.clone();
        }
        if let Some(&collect) = vowels.get(&replace_vowels(&s)) {
            return collect.clone();
        }
        String::new()
    };

    queries.iter().map(conditional_check).collect()

    }
}
