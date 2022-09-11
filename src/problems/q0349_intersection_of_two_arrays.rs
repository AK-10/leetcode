use std::collections::HashMap;

struct Solution {}

impl Solution {
    pub fn intersection(nums1: Vec<i32>, nums2: Vec<i32>) -> Vec<i32> {
        let mut h = HashMap::<i32, i32>::new();
        nums1.iter().for_each(|n| {
            h.insert(*n, 0);
        });
        nums2.iter().for_each(|n| {
            let v = h.get(n);
            match v {
                Some(v) => {
                    h.insert(*n, v + 1);
                }
                None => {}
            }
        });

        h.iter()
            .filter_map(|(k, v)| if *v > 0 { Some(*k) } else { None })
            .collect()
    }
}

#[cfg(test)]
mod test {
    #[test]
    fn test() {
        use crate::problems::q0349_intersection_of_two_arrays::Solution;

        let nums1 = vec![1, 2, 2, 1];
        let nums2 = vec![2, 2];
        assert_eq!(Solution::intersection(nums1, nums2), vec![2]);

        let nums1 = vec![9, 4, 9, 8, 4];
        let nums2 = vec![9, 4];
        assert_eq!(Solution::intersection(nums1, nums2), vec![9, 4]);
    }
}

