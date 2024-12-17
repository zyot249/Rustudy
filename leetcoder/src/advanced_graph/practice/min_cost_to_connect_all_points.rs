/*
Given:
- array points: points[i] = [xi, yi]
- cost of connecting i and j: |xi - xj| + |yi - yj|

Return:
- min cost to make all points connected

Constraints:
- points len: [1, 1000]
- xi, yi: [-10^6, 10^6]
- all points are distinct
*/
use std::cmp::Ordering;
use std::collections::BinaryHeap;

#[derive(Copy, Clone, Eq, PartialEq)]
struct Edge {
    dest: usize,
    distance: i32,
}

impl Ord for Edge {
    fn cmp(&self, other: &Self) -> Ordering {
        other
            .distance
            .cmp(&self.distance)
            .then_with(|| self.dest.cmp(&other.dest))
    }
}

impl PartialOrd for Edge {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

struct Solution;

impl Solution {
    pub fn min_cost_connect_points(points: Vec<Vec<i32>>) -> i32 {
        let point_num = points.len();
        let mut heap: BinaryHeap<Edge> = BinaryHeap::new();
        let mut dist: Vec<i32> = vec![i32::MAX; point_num];

        heap.push(Edge {
            dest: 0,
            distance: 0,
        });

        let mut cost = 0;
        let mut count = 0;
        while let Some(Edge { dest: u, distance }) = heap.pop() {
            if dist[u] == 0 {
                continue;
            }

            dist[u] = 0;
            cost += distance;
            count += 1;

            if count == point_num {
                break;
            }

            for v in 0..point_num {
                if dist[v] == 0 {
                    continue;
                }

                let dis = Self::manhattan_distance(&points[u], &points[v]);
                if dis < dist[v] {
                    dist[v] = dis;
                    heap.push(Edge {
                        dest: v,
                        distance: dis,
                    });
                }
            }
        }

        cost
    }

    fn manhattan_distance(u: &Vec<i32>, v: &Vec<i32>) -> i32 {
        (u[0] - v[0]).abs() + (u[1] - v[1]).abs()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_min_cost_connect_points() {
        assert_eq!(
            Solution::min_cost_connect_points(vec![
                vec![0, 0],
                vec![2, 2],
                vec![3, 10],
                vec![5, 2],
                vec![7, 0]
            ]),
            20
        );
    }
}
