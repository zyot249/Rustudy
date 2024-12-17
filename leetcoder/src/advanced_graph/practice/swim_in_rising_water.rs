/*
Given:
- n x n integer matrix (grid)
- grid[i][j]: elevation at point (i, j)
- at time t, the depth of water is t
- you can swim from a square to another 4-directionally adjacent squares if and only if the elevation of both squares are at most t
- you can swim infinite distance in 0 time

Return:
- The least time until you can reach (n-1, n-1) from (0, 0)

Constraints:
- n: [1, 50]
- grid[i][j]: [0, n^2)
- grid[i][j] is unique
*/
use std::collections::{BinaryHeap};
use std::cmp::Reverse;

struct Solution;

impl Solution {
    pub fn swim_in_water(grid: Vec<Vec<i32>>) -> i32 {
        let n = grid.len();
        let directions = vec![(0, 1), (0, -1), (1, 0), (-1, 0)];

        let mut pq = BinaryHeap::new();
        let mut min_time = vec![vec![i32::MAX; n]; n];

        pq.push(Reverse((grid[0][0], 0, 0)));
        min_time[0][0] = grid[0][0];

        while let Some(Reverse((time, x, y))) = pq.pop() {
            if x == n - 1 && y == n - 1 {
                return time;
            }

            for &(dx, dy) in &directions {
                let x = x as i32 + dx;
                let y = y as i32 + dy;
                if x >= 0 && x < n as i32 && y >= 0 && y < n as i32 {
                    let x = x as usize;
                    let y = y as usize;
                    let new_time = time.max(grid[x][y]);
                    if new_time < min_time[x][y] {
                        min_time[x][y] = new_time;
                        pq.push(Reverse((new_time, x, y)));
                    }
                }
            }
        }

        -1
    }

    pub fn get_index(n: usize, x: usize, y :usize) -> usize {
        x*n + y
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_swim_in_rising_water() {
        assert_eq!(Solution::swim_in_water(vec![vec![0, 2], vec![1, 3]]), 3);
    }

    #[test]
    fn test_swim_in_rising_water_2() {
        assert_eq!(Solution::swim_in_water(vec![vec![0, 1, 2, 3, 4], vec![24, 23, 22, 21, 5], vec![12, 13, 14, 15, 16], vec![11, 17, 18, 19, 20], vec![10, 9, 8, 7, 6]]), 16);
    }
}
