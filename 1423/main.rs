/// https://dev.to/seanpgallivan/solution-maximum-points-you-can-obtain-from-cards-2no
impl Solution {
    pub fn max_score(card_points: Vec<i32>, k: i32) -> i32 {
        let mut total = card_points.iter().take(k as usize).sum::<i32>();
        let mut best = total;
        let (mut i, mut j) = (k - 1, card_points.len() - 1);
        while i >= 0 {
            total += card_points[j] - card_points[i as usize];
            best = best.max(total);
            i -= 1;
            j -= 1;
        }
        best
    }
}
