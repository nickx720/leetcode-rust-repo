impl Solution {
    fn group_the_people(group_sizes: Vec<i32>) -> Vec<Vec<i32>> {
        let mut output: std::collections::HashMap<usize, Vec<i32>> = std::collections::HashMap::new();
        for (i,val) in group_sizes.iter().enumerate(){               
            let value = output.entry(*val as usize).or_insert(Vec::new());                
            value.push(i as i32);
        }    
        let mut displayOutput:Vec<Vec<i32>> = Vec::new();
        for (key, val) in output.iter() {
            for chunk in val.chunks(*key) {
                displayOutput.push(chunk.to_vec())
            }
          
        }
        displayOutput
    }
}