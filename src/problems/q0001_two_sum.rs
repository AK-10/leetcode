pub struct Solution {}
impl Solution {
    // linear
    // pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
    //     for i in 0 ..= nums.len() - 2 {
    //         for j in i + 1 ..= nums.len() - 1 {
    //             if nums[i] + nums[j] == target {
    //                 return vec![i as i32, j as i32]
    //             }
    //         }
    //     }

    //     return vec![]
    // }

    // use hashmap
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        use std::collections::HashMap;
        let mut h = HashMap::<i32, i32>::new();

        for i in 0 .. nums.len() {
            let complement = target - nums[i];
            if h.contains_key(&complement) {
                return vec![*h.get(&complement).unwrap(), i as i32]
            }
            h.insert(nums[i], i as i32);
        }

        return vec![]
    }
}

