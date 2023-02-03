struct Solution;
impl Solution {
    pub fn intersection(nums1: Vec<i32>, nums2: Vec<i32>) -> Vec<i32> {
        let mut nums: [i32; 1001] = [0; 1001];

        nums1.iter().for_each(|num| {
            if nums[(*num as usize)] < 1 {
                nums[(*num as usize)] += 1;
            }
        });

        nums2.iter().for_each(|num| {
            if nums[(*num as usize)] != 0 {
                nums[(*num as usize)] += 1;
            }
        });

        let mut ret = Vec::<i32>::with_capacity(1001);

        nums.iter().enumerate().for_each(|(i, n)| {
            if *n >= 2 {
                ret.push(i as i32);
            }
        });

        ret
    }
}
