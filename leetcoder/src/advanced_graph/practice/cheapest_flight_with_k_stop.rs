/*
Given:
- n cities connected by flights
- arr flights: flights[i]: [from i, to i, price i]
- src, dst
- at most k stops

Return:
- cheapest price
- no route -> -1

Constraints:
- n: [1, 100]
- flights len: [0, n * (n - 1) / 2]
- from i, to i: [0 , n)
- price i: [1, 10^4]
- only 1 flight between 2 cities
- src != dst
- src, dst, k: [0, n)
*/

/*
    Solution: breath first search to k
*/

use std::cmp::Reverse;
use std::collections::BinaryHeap;

struct Solution;

impl Solution {
    pub fn find_cheapest_price(n: i32, flights: Vec<Vec<i32>>, src: i32, dst: i32, k: i32) -> i32 {
        let mut map = vec![vec![0; n as usize]; n as usize];
        for flight in flights {
            map[flight[0] as usize][flight[1] as usize] = flight[2];
        }
        
        let mut pq = BinaryHeap::new();
        let mut cheapest_trip = vec![vec![i32::MAX; k as usize + 1]; n as usize];

        cheapest_trip[src as usize][0] = 0;
        pq.push(Reverse((0, -1, src as usize)));

        while let Some(Reverse((base_price, base_stop, city))) = pq.pop() {
            if base_stop == k {
                continue;
            }

            let new_stop = base_stop + 1;
            for next_city in 0..n as usize {
                let f_price = map[city][next_city];
                if f_price == 0 {
                    continue;
                }

                let next_price = base_price + f_price;
                if next_price < cheapest_trip[next_city][new_stop as usize] {
                    cheapest_trip[next_city][new_stop as usize] = next_price;
                    pq.push(Reverse((next_price, new_stop, next_city)));
                }
            }
        }

        let mut min = i32::MAX;
        for stop in 0..k as usize + 1 {
            let price = cheapest_trip[dst as usize][stop];
            if price < min {
                min = price;
            }
        }

        if min == i32::MAX {
            -1
        } else {
            min
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_find_cheapest_price_1() {
        assert_eq!(
            Solution::find_cheapest_price(
                3,
                vec![vec![0, 1, 100], vec![1, 2, 100], vec![0, 2, 500]],
                0,
                2,
                1
            ),
            200
        );
    }

    #[test]
    fn test_find_cheapest_price_2() {
        assert_eq!(
            Solution::find_cheapest_price(
                5,
                vec![
                    vec![0, 1, 5],
                    vec![1, 2, 5],
                    vec![0, 3, 2],
                    vec![3, 1, 2],
                    vec![1, 4, 1],
                    vec![4, 2, 1]
                ],
                0,
                2,
                2
            ),
            7
        );
    }

    #[test]
    fn test_find_cheapest_price_3() {
        assert_eq!(
            Solution::find_cheapest_price(
                4,
                vec![
                    vec![0, 1, 100],
                    vec![1, 2, 100],
                    vec![2, 0, 100],
                    vec![1, 3, 600],
                    vec![2, 3, 200]
                ],
                0,
                3,
                1
            ),
            700
        );
    }
}
