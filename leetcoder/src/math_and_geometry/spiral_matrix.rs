/*
Given:
- m x n matrix

Return:
- all elements of matrix in spiral order

Constraints:
- m, n: [1, 10]
- matrix[i][j]: [-100, 100]
*/

struct Solution;

impl Solution {
    pub fn spiral_order(matrix: Vec<Vec<i32>>) -> Vec<i32> {
        // let mut m = matrix.len() as i32;
        // let mut n = matrix[0].len() as i32;

        // let mut res: Vec<i32> = Vec::new();
        // let mut origin = 0;
        // while m > 0 && n > 0 {
        //     for i in origin..origin + n {
        //         res.push(matrix[origin as usize][i as usize]);
        //     }

        //     if m > 1 {
        //         if m > 2 {
        //             for i in (origin + 1)..(origin + m - 1) {
        //                 res.push(matrix[i as usize][(origin + n - 1) as usize]);
        //             }
        //         }

        //         for i in (origin..(origin + n)).rev() {
        //             res.push(matrix[(origin + m - 1) as usize][i as usize]);
        //         }

        //         if m > 2 && n > 1 {
        //             for i in ((origin + 1)..(origin + m - 1)).rev() {
        //                 res.push(matrix[i as usize][origin as usize]);
        //             }
        //         }
        //     }

        //     origin = origin + 1;
        //     m = m - 2;
        //     n = n - 2;
        // }

        let directions :Vec<(i32, i32)> = vec![(0, 1), (1, 0), (0, -1), (-1, 0)];
        let mut steps :Vec<i32> = vec![matrix[0].len() as i32, matrix.len() as i32 - 1];

        let mut res = Vec::new();
        let (mut r, mut c, mut d) = (0, -1, 0);
        while steps[d & 1] > 0 {
            for i in 0..steps[d & 1] {
                r += directions[d as usize].0;
                c += directions[d as usize].1;
                res.push(matrix[r as usize][c as usize]);
            }

            steps[d & 1] -= 1;
            d += 1;
            d %= 4;
        }

        res
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_spiral_order() {
        assert_eq!(
            Solution::spiral_order(vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]]),
            vec![1, 2, 3, 6, 9, 8, 7, 4, 5]
        );
    }
}
