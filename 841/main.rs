impl Solution {
    pub fn can_visit_all_rooms(rooms: Vec<Vec<i32>>) -> bool {
        use std::collections::VecDeque;
        let mut vis:Vec<bool> = vec![false;rooms.len()];
        vis[0] = true;
        let mut stack:VecDeque<usize> = VecDeque::new();
        stack.push_back(0);
        let mut count = 1;
        while !stack.is_empty(){
            if let Some(index) = stack.pop_back(){
                for &item in rooms[index].iter() {
                    let item = item as usize;
                    if !vis[item]{
                        stack.push_back(item);
                        vis[item] = true;
                        count+=1;
                    }
                }
            }
        }
        rooms.len() == count

    }
}
