/*
Given:
- integers a, b

Return:
- sum of a and b

Constraints:
- dont use +, -
- a, b: [-1000, 1000]
*/

struct Solution;

impl Solution {
    pub fn get_sum(mut a: i32, mut b: i32) -> i32 {
        while b != 0 {
            let carry = (a & b) << 1;
            a = a ^ b;
            b = carry;
        }

        if a <= i32::MAX {
            a
        } else {
            !a
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_sum() {
        assert_eq!(Solution::get_sum(1, 2), 3);
    }
}
