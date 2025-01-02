/*
Given:
- integer array: nums
- integer target
- add "+" or "-" to make an expression

Return:
- # of different expressions that equal to target

Constraints:
- nums length: [1, 20]
- nums[i]: [0, 1000]
- sum(nums[i]): [0, 1000]
- target: [-1000, 1000]
*/

struct Solution;

impl Solution {
    pub fn find_target_sum_ways(nums: Vec<i32>, target: i32) -> i32 {
        let mut dp = vec![0; 2001];
        dp[1000] = 1;

        for i in 0..nums.len() {
            for t in 0..2001 {
                let mut new_t = t - nums[i];
                if new_t >= 0 {
                    let x = dp[new_t as usize];
                    println!("{x}");
                    dp[t as usize] += dp[new_t as usize];
                }

                new_t = t + nums[i];
                if new_t <= 2000 {
                    dp[t as usize] += dp[new_t as usize];
                }
            }
        }

        dp[(target + 1000) as usize]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let nums = vec![1, 1, 1, 1, 1];
        let target = 3;
        assert_eq!(Solution::find_target_sum_ways(nums, target), 5);
    }
}
