/*
Given:
- n x n matrix

Return:
- rotate this image by 90 degrees

Constraints:
- modify the matrix only, dont allocate another
- n: [1, 20]
- matrix[i][j]: [-1000, 1000]
*/

/*
0,0 -> 0,3 -> 3,3 -> 3,0
0,1 -> 1,3 -> 3,2 -> 2,0
0,2 -> 2,3 -> 3,1 -> 1,0

1,1 -> 1,2 -> 2,2 -> 2,1
1,1 -> 2,1 -> 2,2 -> 1,2

i,j -> n - 1 - j,i -> n - 1 - i,n - 1 - j -> j,n - 1 - i
*/

struct Solution;


impl Solution {
    pub fn rotate(matrix: &mut Vec<Vec<i32>>) {
        let n = matrix.len();

        //Reverse by four cells
        // for i in 0..n/2 {
        //     for j in i..n - i - 1 {
        //         let tmp = matrix[i][j];
        //         matrix[i][j] = matrix[n - 1 - j][i];
        //         matrix[n - 1 - j][i] = matrix[n - 1 - i][n - 1 - j];
        //         matrix[n - 1 - i][n - 1 - j] = matrix[j][n - 1 - i];
        //         matrix[j][n - 1 - i] = tmp;
        //     }
        // }

        // another solution: reverse + transpose

        // Transpose the matrix
        for i in 0..n {
            for j in i + 1..n {
                let temp = matrix[i][j];
                matrix[i][j] = matrix[j][i];
                matrix[j][i] = temp;
            }
        }
        // Reverse each row
        for row in matrix.iter_mut() {
            row.reverse();
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rotate() {
        let mut matrix = vec![vec![1,2,3],vec![4,5,6],vec![7,8,9]];
        Solution::rotate(&mut matrix);
        assert_eq!(matrix, vec![vec![7,4,1],vec![8,5,2],vec![9,6,3]]);
    }
}