/*
Given:
- array of integer nums

Return:
- subarray with largest sum
- return this sum

Constraints:
- array len: [1, 10^5]
- nums[i]: [-10^4, 10^4]
*/

struct Solution;

impl Solution {
    pub fn max_sub_array(nums: Vec<i32>) -> i32 {
        let mut result :i32 = nums[0];
        let mut pre_max :i32 = nums[0];

        for num in &nums[1..] {
            pre_max = (pre_max + *num).max(*num);
            result = pre_max.max(result);
        }

        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_max_sub_array() {
        assert_eq!(Solution::max_sub_array(vec![-2, 1, -3, 4, -1, 2, 1, -5, 4]), 6);
    }
}
