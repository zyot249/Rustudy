/*
Given:
- n balloons
- array nums: nums[i] is painted number on ith balloon
- burst ith balloon -> get nums[i - 1] * nums[i] * nums[i + 1] coins
- need burst all balloons

Return:
- max coins can collect

Constraints:
- n = nums length
- n: [1, 300]
- nums[i]: [0, 100]
*/

/*
1,3,1,5,8,1

dp[l][r] = max(
    i in [l, r]:
    nums[l-1] * nums[i] * nums[r+1] + dp[l][i-1] + dp[i+1][r]
)
*/

struct Solution;

impl Solution {
    pub fn max_coins(nums: Vec<i32>) -> i32 {
        let n = nums.len();
        let mut new_nums = vec![1];
        for i in 0..n {
            new_nums.push(nums[i]);
        }
        new_nums.push(1);
        
        let mut dp = vec![vec![0; n + 2]; n + 2];

        for l in (1..n + 1).rev() {
            for r in l..n + 1 {
                for i in l..r + 1 {
                    dp[l][r] = dp[l][r].max(new_nums[l-1] * new_nums[i] * new_nums[r+1] + dp[l][i-1] + dp[i+1][r]);
                }
            }
        }

        dp[1][n]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_max_coins() {
        assert_eq!(Solution::max_coins(vec![3,1,5,8]), 167);
    }

    #[test]
    fn test_max_coins_2() {
        assert_eq!(Solution::max_coins(vec![1,5]), 10);
    }
}
