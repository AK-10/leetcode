pub mod problems;

#[cfg(test)]
mod tests {
    #[test]
    fn q0001_two_sum_test() {
        use crate::problems::q0001_two_sum::Solution;

        assert_eq!(Solution::two_sum(vec![1, 2], 3), vec![0, 1]);
        assert_eq!(Solution::two_sum(vec![1, 3, 5, 4], 7), vec![1, 3]);
        assert_eq!(Solution::two_sum(vec![10, 4, 30, 1], 5), vec![1, 3]);
        assert_eq!(Solution::two_sum(vec![2, 7, 11, 15], 9), vec![0, 1]);
    }
}
