/*
Given:
- 2 strings: s and t

Return:
- # of subsequences of s which equals to t

Constraints:
- result is integer
- string length: [1, 1000]
- string contains Eng letters
*/

/*
babgbag
bag

       b a b g b a g
b.     1 1 2 2 3 3 3 
ba.    0 1 1 1 1 4 4
bag.   0 0 0 1 1 1 5


rabbbit
rabbit

        r a b b b i t
r.      1 1 1 1 1 1 1
ra.     0 1 1 1 1 1 1
rab.    0 0 1 2 3 3 3
rabb.   0 0 0 1 3 3 3
rabbi.  0 0 0 0 0 3 3
rabbit. 0 0 0 0 0 0 3
*/

struct Solution;

impl Solution {
    pub fn num_distinct(s: String, t: String) -> i32 {
        let (s, t) = (s.as_bytes(), t.as_bytes());
        let (m, n) = (s.len(), t.len());

        if m < n {
            return 0;
        }

        // let mut dp = vec![vec![0; m + 1]; n + 1];
        // dp[0] = vec![1; m + 1];

        // for i in 1..n + 1 {
        //     for j in 1..m + 1 {
        //         dp[i][j] = dp[i][j - 1];
        //         if t[i - 1] == s[j - 1] {
        //             dp[i][j] += dp[i - 1][j - 1];
        //         }
        //     }
        // }

        // dp[n][m]

        //Optimized solution

        let mut dp = vec![1; m];
        let mut prev;

        for i in 0..n {
            let mut curr = if i == 0 {1} else {0};
            for j in 0..m {
                prev = curr;
                curr = dp[j];

                dp[j] = if j > 0 {
                    dp[j - 1]
                } else {
                    0
                };

                if t[i] == s[j] {
                    dp[j] += prev;
                }
            }
        }

        dp[m - 1]

        // best solution
        // let s = s.into_bytes();
        // let t = t.into_bytes();

        // let m = s.len();
        // let n = t.len();

        // let mut d = vec![0; n + 1];
        // d[0] = 1;

        // for i in 1..=m {
        //     let x = n.saturating_sub(m - i).max(1);
        //     for j in (x..=n).rev() {
        //         d[j] += (s[i - 1] == t[j - 1]) as i32 * d[j - 1];
        //     }
        // }

        // d[n]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_num_distinct() {
        assert_eq!(Solution::num_distinct("rabbbit".to_string(), "rabbit".to_string()), 3);
    }

    #[test]
    fn test_num_distinct_2() {
        assert_eq!(Solution::num_distinct("babgbag".to_string(), "bag".to_string()), 5);
    }

    #[test]
    fn test_num_distinct_3() {
        assert_eq!(Solution::num_distinct("a".to_string(), "a".to_string()), 1);
    }
}
