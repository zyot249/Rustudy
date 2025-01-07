/*
Given:
- strings: word1, word2
- permitted operations:
    - insert
    - delete
    - replace

Return:
- min number of operations required to convert word1 to word2

Constraints:
- string length: [1, 500]
- string consists of lowercase Eng letters
*/

/*
ros
at i:
- insert if word[i] = word2[i + 1]
- remove if word[i + 1] = word2[i] || word2[i] not exist
- replace if word[i] != word2[i]
- do nothg

horse
-> rorse -> rose -> ros

base cases:
"" -> "" : 0

inten -> execu
    0 1 2 3
      r o s
0   0 
1 h 1
2 o 2
3 r 3
4 s 4
5 e 5
h -> r: 
 - replace: 1 + ("" -> "")
 - insert: 1 + ("h" -> "")
 - remove: 1 + ("" -> "r")
*/

struct Solution;

impl Solution {
    pub fn min_distance(word1: String, word2: String) -> i32 {
        let (word1, word2) = (word1.into_bytes(), word2.into_bytes());
        let (m, n) = (word1.len(), word2.len());

        let mut dp = vec![vec![i32::MAX; n + 1]; m + 1];
        dp[0][0] = 0;

        for j in 0..n + 1 {
            for i in 0..m + 1 {
                if i > 0 {
                    dp[i][j] = dp[i][j].min(1 + dp[i - 1][j]);
                }

                if j > 0 {
                    dp[i][j] = dp[i][j].min(1 + dp[i][j - 1]);
                }

                if i > 0 && j > 0 {
                    if word1[i - 1] == word2[j - 1] {
                        dp[i][j] = dp[i][j].min(dp[i - 1][j - 1]);
                    } else {
                        dp[i][j] = dp[i][j].min(1 + dp[i - 1][j - 1]);
                    }
                }
            }
        }

        dp[m][n]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_min_distance() {
        assert_eq!(Solution::min_distance("horse".to_string(), "ros".to_string()), 3);
    }

    #[test]
    fn test_min_distance_2() {
        assert_eq!(Solution::min_distance("intention".to_string(), "execution".to_string()), 5);
    }
}
