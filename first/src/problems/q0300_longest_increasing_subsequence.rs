pub struct Solution {}

// 1 <= nums.len <= 2500
// 10^-4 <= n <= 10^4
impl Solution {
    pub fn length_of_lis(nums: Vec<i32>) -> i32 {
        let mut max_lis = 1;
        let mut dp = vec![1; nums.len()];

        for i in 0..nums.len() {
            for j in 0..i {
                if nums[j] < nums[i] {
                    dp[i] = dp[i].max(dp[j] + 1);
                    max_lis = max_lis.max(dp[i]);
                }
            }
        }

        max_lis
    }
}

#[cfg(test)]
mod tests {
    use crate::problems::q0300_longest_increasing_subsequence::Solution;

    #[test]
    fn test() {
        let input = vec![10, 9, 2, 5, 3, 7, 101, 18];
        let expected = 4; // [2, 3, 7, 101]
        assert_eq!(Solution::length_of_lis(input), expected);

        let input = vec![0, 1, 0, 3, 2, 3];
        let expected = 4; // [0, 1, 2, 3]
        assert_eq!(Solution::length_of_lis(input), expected);
    }
}
