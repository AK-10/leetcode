
struct Solution {}
impl Solution {
    pub fn move_zeroes(nums: &mut Vec<i32>) {
        let mut zero_cnt = 0;
        let mut index = 0;
        while index < nums.len() {
            if nums[index] == 0 {
                nums.remove(index);
                zero_cnt += 1;
            } else {
                index += 1;
            }
        }

        for _ in 0..zero_cnt {
            nums.push(0);
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::problems::q0283_move_zeros::Solution;

    #[test]
    fn test() {
        let mut input = vec![0,1,0,3,12];
        let expected = vec![1, 3, 12, 0, 0];
        Solution::move_zeroes(&mut input);
        assert_eq!(input , expected);

        let mut input = vec![0];
        let expected = vec![0];
        Solution::move_zeroes(&mut input);
        assert_eq!(input , expected);

        let mut input = vec![1, 2, 3];
        let expected = vec![1, 2, 3];
        Solution::move_zeroes(&mut input);
        assert_eq!(input , expected);

        let mut input = vec![0, 0, 1];
        let expected = vec![1, 0, 0];
        Solution::move_zeroes(&mut input);
        assert_eq!(input , expected);
    }
}

// [0, 1, 1, 1] j = 0, i = 0
// [0, 1, 1, 1] j = 0
// [1, 0, 1, 1]
// [1, 1, 0, 1]
//
//
// [0, 0, 1, 1] j = 0, i = 0
// [0, 0 ,1 ,1] j = 0, i = 1
// [1, 0, 0, 1] j = 1, i = 2
// [1, 1, 0, 0] j = 2, i = 3
