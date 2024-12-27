/*
Given:
- 2 strings: text1, text2
- A subsequence of a string is a new string generated from the original string with some characters (can be none) deleted without changing the relative order of the remaining characters.

For example, "ace" is a subsequence of "abcde".

Return:
- length of longest common subsequence
- 0 if no common subsequence

Constraints:
- string length: [1, 1000]
- string contains lowercase English characters only
*/

/*
    let text1 = "abcde";
    let text2 = "acede";

        a b c d e
      0 0 0 0 0 0
    a 0 1 1 1 1 1
    c 0 1 1 2 2 2
    e 0 1 1 2 2 3
    d 0 1 1 2 3 3
    e 0 1 1 2 3 4
*/

struct Solution;

impl Solution {
    // pub fn longest_common_subsequence(text1: String, text2: String) -> i32 {
    //     let mut dp = vec![vec![0; text1.len() + 1]; text2.len() + 1];
    //     let text1 = text1.as_bytes();
    //     let text2 = text2.as_bytes();

    //     for i in 1..text2.len() + 1 {
    //         for j in 1..text1.len() + 1 {
    //             if text2[i - 1] == text1[j - 1] {
    //                 dp[i][j] = dp[i - 1][j - 1] + 1;
    //             } else {
    //                 dp[i][j] = dp[i][j - 1].max(dp[i - 1][j]);
    //             }
    //         }
    //     }

    //     dp[text2.len()][text1.len()]
    // }

    pub fn longest_common_subsequence(text1: String, text2: String) -> i32 {
        let mut dp = vec![0; text2.len() + 1];
        let text1 = text1.as_bytes();
        let text2 = text2.as_bytes();

        for i in 1..text1.len() + 1 {
            let mut prev;
            let mut curr = 0;

            for j in 1..text2.len() + 1 {
                prev = curr;
                curr = dp[j];
                if text2[j - 1] == text1[i - 1] {
                    dp[j] = prev + 1;
                } else {
                    dp[j] = dp[j].max(dp[j - 1]);
                }
            }
        }

        dp[text2.len()]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solution_1(){
        let text1 = "abcde";
        let text2 = "ace";

        assert_eq!(Solution::longest_common_subsequence(text1.to_string(), text2.to_string()), 3);
    }

    #[test]
    fn test_solution_2(){
        let text1 = "abc";
        let text2 = "abc";

        assert_eq!(Solution::longest_common_subsequence(text1.to_string(), text2.to_string()), 3);
    }

    #[test]
    fn test_solution_3(){
        let text1 = "abc";
        let text2 = "def";

        assert_eq!(Solution::longest_common_subsequence(text1.to_string(), text2.to_string()), 0);
    }

    #[test]
    fn test_solution_4(){
        let text1 = "abcde";
        let text2 = "acede";

        // it should be "acde"
        assert_eq!(Solution::longest_common_subsequence(text1.to_string(), text2.to_string()), 4);
    }

    #[test]
    fn test_solution_5(){
        let text1 = "abc";
        let text2 = "cba";

        // it should be "acde"
        assert_eq!(Solution::longest_common_subsequence(text1.to_string(), text2.to_string()), 1);
    }

    #[test]
    fn test_solution_6(){
        let text1 = "abc";
        let text2 = "acd";

        // it should be "acde"
        assert_eq!(Solution::longest_common_subsequence(text1.to_string(), text2.to_string()), 2);
    }

    #[test]
    fn test_solution_7(){
        let text1 = "aggtab";
        let text2 = "gxtxayb";

        // it should be "acde"
        assert_eq!(Solution::longest_common_subsequence(text1.to_string(), text2.to_string()), 4);
    }
}