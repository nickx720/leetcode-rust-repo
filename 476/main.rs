impl Solution {
    pub fn find_complement(num: i32) -> i32 {
         let num = format!("{:b}",num).chars().map(|x| { if x == '1' {
          '0'} else { '1'} }).collect::<Vec<char>>().into_iter().collect::<String>();
      let num = i32::from_str_radix(&num, 2);
      if let Ok(output) = num {
          return output;
      } else {
          unreachable!()
      }
    }
}
