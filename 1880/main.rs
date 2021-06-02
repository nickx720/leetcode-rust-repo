impl Solution {
    pub fn is_sum_equal(first_word: String, second_word: String, target_word: String) -> bool {
        let char_to_num = |c: char| -> u32 { c as u32 - 97 };
    let first_word = first_word
        .chars()
        .map(char_to_num)
        .enumerate()
        .fold(0, |mut acc, (ind, elem)| {
            acc += elem * 10u32.pow((first_word.len() - ind-1) as u32);
            acc
        });
    let second_word = second_word
        .chars()
        .map(char_to_num)
        .enumerate()
        .fold(0, |mut acc, (ind, elem)| {
            acc += elem * 10u32.pow((second_word.len() - ind-1) as u32);
            acc
        });
    let  target_word= target_word
        .chars()
        .map(char_to_num)
        .enumerate()
        .fold(0, |mut acc, (ind, elem)| {
            acc += elem * 10u32.pow((target_word.len() - ind-1) as u32);
            acc
        });
    first_word+second_word == target_word
    }
}