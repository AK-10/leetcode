struct Solution {}

impl Solution {
    pub fn search_insert(nums: Vec<i32>, target: i32) -> i32 {
        let mut l: i32 = 0;
        let mut r: i32 = (nums.len() - 1) as i32;

        while l <= r {
            // min
            if target < nums[l as usize] {
                return l;
            }
            // max
            if target > nums[r as usize] {
                return r + 1;
            }

            let index: i32 = l + (r - l) / 2;
            if  target < nums[index as usize] {
                r = index - 1;
            } else if target > nums[index as usize] {
                l = index + 1;
            } else { // target == index
                return index;
            }
        }

        return l;
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test() {
        use crate::problems::q0035_search_insert_position::Solution;

        let nums = vec![1, 3, 5, 6];

        let input = 5;
        let expected = 2;
        assert_eq!(Solution::search_insert(nums.clone(), input), expected);

        let input = 2;
        let expected = 1;
        assert_eq!(Solution::search_insert(nums.clone(), input), expected);

        let input = 7;
        let expected = 4;
        assert_eq!(Solution::search_insert(nums.clone(), input), expected);
    }
}
