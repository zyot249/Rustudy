/*
Given:
- strings: s1, s2, s3

Return:
- s3 is interleaving of s1, s2?

Constraints:
- s1, s2 length: [0, 100]
- s3 length: [0, 200]
- string consist of lowercase Eng letters only

Optimization: O(s2.length) space complexity
*/

/*
aabcc
dbbca
aadbbcbcac

0 0
1 0
2 0
2 1
2 2
2 3
2 4
3 4
4 4
4 5
5 5

  0 1 2 3 4 5
0 1 0 0
1 0 0
2
3
4
5

directions: (0, 1), (1, 0)

m * n

*/

struct Solution;

impl Solution {
    pub fn is_interleave(s1: String, s2: String, s3: String) -> bool {
        // let s1 = s1.as_bytes();
        // let s2 = s2.as_bytes();
        // let s3 = s3.as_bytes();
        
        // let m = s1.len();
        // let n = s2.len();
        // if s3.len() != m + n {
        //     return false;
        // }

        // let mut dp = vec![vec![false; n + 1]; m + 1];
        // dp[0][0] = true;
        // for i in 0..m + 1 {
        //     for j in 0..n + 1 {
        //         if i >= 1 && s1[i - 1] == s3[i + j - 1] {
        //             dp[i][j] |= dp[i - 1][j];
        //         }

        //         if j >= 1 && s2[j - 1] == s3[i + j - 1] {
        //             dp[i][j] |= dp[i][j - 1];
        //         }
        //     }
        // }

        // dp[m][n]

        // Optimize
        let s1 = s1.as_bytes();
        let s2 = s2.as_bytes();
        let s3 = s3.as_bytes();
        
        let m = s1.len();
        let n = s2.len();
        if s3.len() != m + n {
            return false;
        }

        if n == 0 {
            return s1 == s3;
        }

        let mut dp = vec![false; n + 1];
        for i in 0..m + 1 {
            for j in 0..n + 1 {
                if i == 0 && j == 0 {
                    dp[j] = true;
                    continue;
                }

                let mut result = false;
                if i >= 1 && s1[i - 1] == s3[i + j - 1] {
                    result |= dp[j];
                }

                if j >= 1 && s2[j - 1] == s3[i + j - 1] {
                    result |= dp[j - 1];
                }

                dp[j] = result;
            }
        }

        dp[n]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_interleave() {
        assert!(Solution::is_interleave("aabcc".to_string(), "dbbca".to_string(), "aadbbcbcac".to_string()));
    }

    #[test]
    fn test_is_interleave_2() {
        assert!(!Solution::is_interleave("aabcc".to_string(), "dbbca".to_string(), "aadbbbaccc".to_string()));
    }

    #[test]
    fn test_is_interleave_3() {
        assert!(Solution::is_interleave("".to_string(), "".to_string(), "".to_string()));
    }
}
