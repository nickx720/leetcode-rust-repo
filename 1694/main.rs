impl Solution {
    pub fn reformat_number(number: String) -> String {
        let mut number_arr:Vec<u32> = Vec::new();
        for item in number.chars(){
            if item.is_digit(10){
                if let Some(number) = item.to_digit(10){
                    number_arr.push(number);
                }
            }
        }
        let mut iter:Vec<Vec<u32>>  = number_arr[..].chunks(3).map(|x| x.to_owned()).collect();
        if iter.len() >= 2 {
            let iter_len = iter.len()-2;
            let iter_len_last = iter.len()-1;
            if let Some(mut last) = iter.last(){
                if last.len() == 1 {
                    if let Some(item) = &iter[iter_len].pop(){
                        iter[iter_len_last].insert(0, *item);
                    };
                }
            }
        }
        iter.iter().map(|x| x.iter().map(ToString::to_string).collect::<String>()).collect::<Vec<String>>().join("-")

    }
}
