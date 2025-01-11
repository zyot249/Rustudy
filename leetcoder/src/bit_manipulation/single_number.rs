/*
Given:
- non-empty integer array: nums
- every element appears twice except 1

Return:
- the element that appears 1

Constraints:
- nums length: [1, 3*10^4]
- nums[i]: [-3*10^4 3*10^4]
*/

struct Solution;

impl Solution {
    pub fn single_number(nums: Vec<i32>) -> i32 {
        let mut res = 0;
        for i in 0..nums.len() {
            res ^= nums[i];
        }

        res
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_single_number() {
        assert_eq!(Solution::single_number(vec![2,2,1]), 1);
    }

    #[test]
    fn test_single_number_2() {
        assert_eq!(Solution::single_number(vec![3,1,2,1,2]), 3);
    }
}