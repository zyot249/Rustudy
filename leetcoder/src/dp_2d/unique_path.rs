/*
Given:
- grid m x n
- robot starts at grid[0][0]
- robot tries to move to grid[m - 1][n - 1]
- robot can only move either down or right

Return:
- number of possible unique paths

Constraints:
- Result is integer
- m, n: [1, 100]
*/

struct Solution;

impl Solution {
    pub fn unique_paths(m: i32, n: i32) -> i32 {
        let m = m as usize;
        let n = n as usize;
        let mut grid = vec![vec![0; n]; m];
        grid[0][0] = 1;

        for x in 0..m {
            for y in 0..n {
                if x > 0 {
                    grid[x][y] += grid[x - 1][y];
                }

                if y > 0 {
                    grid[x][y] += grid[x][y - 1];
                }
            }
        }

        grid[m - 1][n - 1]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_unique_paths() {
        assert_eq!(Solution::unique_paths(3, 7), 28);
    }
}
