/*
Given:
- positive integer n
- A set bit refers to a bit in the binary representation of a number that has a value of 1.

Return:
- number of set bits in its binary representation

Contraints:
- n: [1, 2^31 -1]
*/

struct Solution;

impl Solution {
    pub fn hamming_weight(mut n: i32) -> i32 {
        let mut count = 0;
        while n > 0 {
            if n%2 != 0 {
                count += 1;
            }

            n = n >> 1
        }

        count
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hamming_weight() {
        assert_eq!(Solution::hamming_weight(1), 1);
    }
}