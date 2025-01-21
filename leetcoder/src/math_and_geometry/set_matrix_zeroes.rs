/*
Given:
- m x n integer matrix

Return:
- if an element is 0 --> set its entire rol and col to 0

Constraints:
- m, n: [1, 200]
- matrix[i][j]: [2^-31, 2^31 -1]
- improve space O(mn) -> O(m + n) -> O(1)
- must be done in place
*/

/*
1: 
loop all element --> meet 0 --> add all elements in entire rol and col to a set
time: m*n*(m + n) + m*n
space: m*n

2:
loop all element --> meet 0 --> add entire rol and col to corresponding set
time: m*n + m*n*(m + n)
space: m + n

3:
use row 0 and col 0 to cache rows and cols that need to be 0
loop all element --> meet 0
*/

struct Solution;

impl Solution {
    pub fn set_zeroes(matrix: &mut Vec<Vec<i32>>) {
        let (m, n) = (matrix.len(), matrix[0].len());
        let mut row_zero = false;

        for i in 0..m {
            for j in 0..n {
                if matrix[i][j] == 0 {
                    matrix[0][j] = 0;

                    if i > 0 {
                        matrix[i][0] = 0;
                    } else {
                        row_zero = true;
                    }
                }
            }
        }

        for i in 1..m {
            for j in 1..n {
                if matrix[0][j] == 0 || matrix[i][0] == 0 {
                    matrix[i][j] = 0;
                }
            }
        }

        if matrix[0][0] == 0 {
            for i in 0..m {
                matrix[i][0] = 0;
            }
        }

        if row_zero {
            for j in 0..n {
                matrix[0][j] = 0;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_set_zeroes() {
        let mut matrix = vec![vec![1,1,1],vec![1,0,1],vec![1,1,1]];
        Solution::set_zeroes(&mut matrix);
        assert_eq!(matrix, vec![vec![1,0,1],vec![0,0,0],vec![1,0,1]]);
    }
}