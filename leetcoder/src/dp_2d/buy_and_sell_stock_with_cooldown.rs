/*
Given:
- an array prices: prices[i]: price of stock on ith day

Return:
- max profit

Constraints:
- after sell stock, cannot buy on the next day
- must sell before buy again
- prices length: [1, 5000]
- prices[i]: [1, 1000]
*/

/*
1,2,3,0,2

0 B S
1
2
3
4
5

dp[i, B] = max(dp[i + 1, B], dp[i + 1, S] - prices[i])
dp[i, S] = max(dp[i + 1, S], dp[i + 2, B] + prices[i])

final result is dp[0, B] since cannot have dp[0, S]

*/

/*
Note:
- try to go from start to end of prices
- for each day, we can be in 3 states:
    - can buy
    - can sell
    - cooldown
- we can transition between states
*/

struct Solution;

impl Solution {
    pub fn max_profit(prices: Vec<i32>) -> i32 {
        let mut dp = vec![0, 0];

        let mut prev;
        let mut curr = 0;
        for i in (0..prices.len()).rev() {
            prev = curr;
            curr = dp[0];

            dp[0] = dp[0].max(dp[1] - prices[i]);
            dp[1] = dp[1].max(prev + prices[i]);
        }

        dp[0]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_max_profit_1() {
        assert_eq!(Solution::max_profit(vec![1, 2, 3, 0, 2]), 3);
    }

    #[test]
    fn test_max_profit_2() {
        assert_eq!(Solution::max_profit(vec![1]), 0);
    }

    #[test]
    fn test_max_profit_3() {
        assert_eq!(Solution::max_profit(vec![1, 2]), 1);
    }
}
