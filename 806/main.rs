impl Solution {
    pub fn number_of_lines(widths: Vec<i32>, s: String) -> Vec<i32> {
      let s = s.bytes().map(|x| (x - 97) as usize).collect::<Vec<usize>>();
      let (mut lines, mut width) = (1,0);
      for index in s {
          let w = widths[index];
          width+=w;
          if width > 100 {
              lines+=1;
              width=w;
          }
      }
      vec![lines,width as i32]  
    }
}
