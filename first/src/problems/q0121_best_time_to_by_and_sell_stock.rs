struct Solution {}

// buy -> sell
// input example: [7,1,5,3,6,4]
// output example: 5 (buy: 1, sell: 6)
impl Solution {
    fn max_profit(prices: Vec<i32>) -> i32 {
        let mut least = std::i32::MAX;
        let mut dp = vec![0; prices.len() + 1];
        dp[0] = 0;

        for i in 1..=prices.len() {
            let price = prices[i - 1];
            if price < least {
                least = price;
            }
            dp[i] = std::cmp::max(dp[i - 1], price - least);
        }

        dp[prices.len()]
    }
    // first answer
    // too slow.
    // Θ(N^2)
    // pub fn too_slow_max_profit(prices: Vec<i32>) -> i32 {
    //     let mut result = 0;

    //     for (i, buy_price) in prices.iter().enumerate() {
    //         for sell_price in prices[i..].iter() {
    //             let profit = sell_price - buy_price;
    //             if profit > result {
    //                 result = profit;
    //             }
    //         }
    //     }

    //     result
    // }
}

#[cfg(test)]
mod tests {
    use crate::problems::q0121_best_time_to_by_and_sell_stock::Solution;

    #[test]
    fn test() {
        let input = vec![7,1,5,3,6,4];
        let expected = 5;
        assert_eq!(Solution::max_profit(input), expected);

        let input = vec![7,6,4,3,1];
        let expected = 0;
        assert_eq!(Solution::max_profit(input), expected);
    }
}
