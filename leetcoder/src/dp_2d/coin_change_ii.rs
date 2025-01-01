/*
Given:
- array of coins: coin denominations
- amount: total amount of money

Return:
- # of combinations that make up that amout

Constraints:
- the answer is i32
- coins length: [1, 300]
- coins[i]: [1, 5000]
- amount: [0, 5000]
*/

/*
5 [1, 2, 5]

dp[a, i] = dp[a, i - 1] + dp[a - coins[i - 1], i]
*/

struct Solution;

impl Solution {
    pub fn change(amount: i32, coins: Vec<i32>) -> i32 {
        // let mut dp = vec![vec![0; coins.len() + 1]; amount as usize + 1];
        // dp[0] = vec![1; coins.len() + 1];

        // for m in 1..amount as usize + 1 {
        //     for i in 1..coins.len() + 1 {
        //         dp[m][i] = dp[m][i - 1];
        //         if m >= coins[i - 1] as usize {
        //             dp[m][i] += dp[m - coins[i - 1] as usize][i];
        //         }
        //     }
        // }
 
        // dp[amount as usize][coins.len()]

        let amount = amount as usize;
        let mut dp = vec![0; amount + 1];
        dp[0] = 1;

        for i in 0..coins.len() {
            for m in 1..amount + 1 {
                if coins[i] as usize <= m {
                    dp[m] += dp[m - coins[i] as usize];
                }
            }
        }

        dp[amount]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_change() {
        assert_eq!(Solution::change(5, vec![1, 2, 5]), 4);
    }
}
