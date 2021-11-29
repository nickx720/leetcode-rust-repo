impl Solution {
    pub fn max_coins(piles: Vec<i32>) -> i32 {
        let mut piles = piles;
        let (mut sum, mut i, mut j) = (0, piles.len() - 2, 0);
        piles.sort();
        while j < piles.len() / 3 {
            sum += piles[i];
            i -= 2;
            j += 1;
        }
        sum
    }
}
