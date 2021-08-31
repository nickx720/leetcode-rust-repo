// https://dev.to/seanpgallivan/solution-advantage-shuffle-p39
impl Solution {
    pub fn advantage_count(nums1: Vec<i32>, nums2: Vec<i32>) -> Vec<i32> {
        let mut nums = nums1;
        let mut ord:Vec<usize> = (0..nums2.len()).collect();
        let mut ans:Vec<i32> = vec![0;nums.len()];
        ord.sort_by(|&a,&b| nums2.get(b).cmp(&nums2.get(a)));
        nums.sort();
        let (mut i, mut j) = (0, nums2.len()-1);
        for item in ord {
            if nums.get(j) > nums2.get(item){
                ans[item] = nums[j];
                if j == 0 {
                    continue;
                }
                j-=1;
            } else {
                ans[item] = nums[i];
                i+=1;
            }
        }
        ans

    }
}
