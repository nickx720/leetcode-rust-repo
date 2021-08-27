impl Solution {
    pub fn find_ocurrences(text: String, first: String, second: String) -> Vec<String> {
        let text:Vec<&str> = text.split_whitespace().collect();
        let mut output = Vec::new();
        for index in 0..text.len()-1{
            if text[index] == first && text[index+1] == second{
                if index + 2 < text.len() {
                    output.push(text[index+2].to_string());
                }
            }
        }
        output

    }
}
