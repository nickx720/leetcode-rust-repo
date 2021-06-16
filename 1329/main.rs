impl Solution {
    pub fn diagonal_sort(mat: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
       use std::collections::{HashMap,BinaryHeap};
    // Hashmap to store keys and Binary Heap to sort it
    let mut mat = mat;
    let row_index = mat.len();
    let col_index = mat[0].len();

    let mut map:HashMap<i32,BinaryHeap<i32>> = HashMap::new();

    for i in 0..row_index{ 
        for j in 0..col_index{ 
            map.entry(i as i32 - j as i32).or_default().push(-mat[i][j])
        }
    }
    for i in 0..row_index{ 
        for j in 0..col_index { 
            mat[i][j] = -map.entry(i as i32 - j as i32).or_default().pop().unwrap();
        }
    }
    mat  
  
    }
}
