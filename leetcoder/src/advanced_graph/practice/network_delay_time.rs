/*
Given:
- n nodes in network (1 -> n)
- times[i] = (ui, vi, wi): time from ui to vi is wi
- node k
Return:
- min time for all n nodes receive signal from k
- if it is not possible for all n nodes to receive the signal --> -1

Constraints:
- k, n: [1, 100]
- times len: [1, 6000]
- ui != vi
- wi: [0, 100]
*/
use std::collections::BinaryHeap;
use std::cmp::Reverse;

struct Solution;

impl Solution {
    pub fn network_delay_time(times: Vec<Vec<i32>>, n: i32, k: i32) -> i32 {
        let mut network :Vec<Vec<i32>> = vec![vec![-1; n as usize]; n as usize];
        for time in times {
            network[time[0] as usize - 1][time[1] as usize - 1] = time[2];
        }

        let mut min_times :Vec<i32> = vec![i32::MAX; n as usize];
        let mut heap = BinaryHeap::new();
        min_times[k as usize - 1] = 0;
        heap.push(Reverse((0, k as usize - 1)));

        while let Some(Reverse((base_time, node))) = heap.pop() {
            let adjacents = &network[node];
            for neighbor in 0..adjacents.len() {
                if adjacents[neighbor] == -1 {
                    continue;
                }

                let time = base_time + adjacents[neighbor];
                if time < min_times[neighbor] {
                    min_times[neighbor] = time;
                    heap.push(Reverse((time, neighbor)));
                }
            }
        }

        let mut result = 0;
        for time in min_times {
            if time == i32::MAX {
                return -1;
            }

            result = result.max(time);
        }

        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_network_delay_time() {
        assert_eq!(Solution::network_delay_time(vec![vec![2, 1, 1], vec![2, 3, 1], vec![3, 4, 1]], 4, 2), 2);
    }
}
