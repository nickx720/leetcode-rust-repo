impl Solution {
    pub fn deck_revealed_increasing(deck: Vec<i32>) -> Vec<i32> {
        use std::collections::VecDeque;
    let mut deck = deck;
    deck.sort();
    let mut state = VecDeque::new();
    for i in (0..deck.len()).rev() {
        if let Some(back) = state.pop_back() {
            state.push_front(back);
        }
        state.push_front(deck[i]);
    }
    Vec::from(state)
    }
}