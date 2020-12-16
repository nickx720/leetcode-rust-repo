fn unique_morse_representations(words: Vec<String>) -> i32 {
    let morse_codes = vec![
        ".-", "-...", "-.-.", "-..", ".", "..-.", "--.", "....", "..", ".---", "-.-", ".-..", "--",
        "-.", "---", ".--.", "--.-", ".-.", "...", "-", "..-", "...-", ".--", "-..-", "-.--",
        "--..",
    ];

    let mut morse_len = std::collections::HashSet::new();
    for word in &words {
        let mut temp_str: String = "".into();
        let mut chars = word.chars();
        let mut each_char = chars.next();
        loop {
            match each_char {
                Some(x) => temp_str.push_str(morse_codes[(x as u8 - 97) as usize]),
                None => break,
            }
            each_char = chars.next();
        }
        morse_len.insert(temp_str);
    }
    morse_len.len() as i32
}
