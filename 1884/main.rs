impl Solution {
    pub fn two_egg_drop(n: i32) -> i32 {
        let floors = n as usize;
      let eggs = 2;
    let mut count;
      let mut output: Vec<Vec<i32>> = vec![vec![0;floors+1];eggs+1];
      for index in 0..=floors {
          output[1][index] = index as i32;
      }

      for index in 1..=floors {
          output[2][index] = std::i32::MAX;
          for offset in 1..=index {
              count = 1+std::cmp::max(output[1][offset-1],output[2][index-offset]);
              if count < output[2][index]{
                  output[2][index] = count;
              }
          }
      }

      output[eggs][floors]
    }
}
