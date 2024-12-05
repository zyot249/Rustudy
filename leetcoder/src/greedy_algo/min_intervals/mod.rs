/*
Given:
- array intervals
    - interval[i]: [lefti, righti]
    - interval[i] size: righti- lefti + 1
- array queries:
    - answer to query jth is size of smallest interval i: lefti <= queries[j] <= righti
    - if no answer --> -1

Return:
- array of answer

Constraints:
- intervals len: [1, 10^5]
- queries len: [1, 10^5]
- lefti <= righti: [1, 10^7]
- queries[j]: [1, 10^7]
*/

use core::cmp::Ordering;
use std::collections::{BinaryHeap, HashMap};

#[derive(PartialEq, Eq)]
struct IntervalInHeap {
    size: i32,
    right: i32,
}

impl IntervalInHeap {
    fn new(_size: i32, _right: i32) -> Self {
        IntervalInHeap {
            size: _size,
            right: _right,
        }
    }
}

impl Ord for IntervalInHeap {
    fn cmp(&self, other: &Self) -> Ordering {
        if self.size == other.size {
            other.right.cmp(&self.right)
        } else {
            other.size.cmp(&self.size)
        }
    }
}

impl PartialOrd for IntervalInHeap {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

pub struct Solution;

impl Solution {
    pub fn min_interval(mut intervals: Vec<Vec<i32>>, queries: Vec<i32>) -> Vec<i32> {
        intervals.sort_by(|a, b| a[0].cmp(&b[0]));
        let mut sorted_queries = queries.clone();
        sorted_queries.sort();

        let mut answers = HashMap::new();
        let mut min_heap = BinaryHeap::<IntervalInHeap>::new();

        let mut interval_idx = 0;
        for query_idx in 0..sorted_queries.len() {
            let query = sorted_queries[query_idx].clone();
            if query_idx > 0 && query == sorted_queries[query_idx - 1] {
                continue;
            }

            while interval_idx < intervals.len() {
                let interval = &intervals[interval_idx];
                if interval[0] > query {
                    break;
                }

                if interval[1] >= query {
                    min_heap.push(IntervalInHeap::new(
                        Solution::size_of_interval(interval),
                        interval[1].clone(),
                    ));
                }

                interval_idx += 1;
            }

            let mut found_ans = false;
            while min_heap.len() > 0 {
                match min_heap.peek() {
                    Some(peek) => {
                        if peek.right < query {
                            min_heap.pop();
                            continue;
                        } else {
                            answers.insert(query, peek.size.clone());
                            found_ans = true;
                            break;
                        }
                    }
                    None => continue,
                }
            }

            if !found_ans {
                answers.insert(query, -1);
            }
        }

        let mut result = Vec::<i32>::new();
        for query in queries {
            match answers.get(&query) {
                Some(ans) => result.push(ans.clone()),
                None => continue,
            }
        }

        result
    }

    fn size_of_interval(interval: &Vec<i32>) -> i32 {
        interval[1] - interval[0] + 1
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_min_interval() {
        let intervals = vec![
            vec![4, 5],
            vec![5, 8],
            vec![1, 9],
            vec![8, 10],
            vec![1, 6],
            vec![7, 9],
            vec![3, 3],
            vec![3, 5],
            vec![1, 6],
            vec![7, 7],
        ];
        let queries = vec![2, 2, 6, 3, 9, 6, 1, 1, 1, 9];
        let result = Solution::min_interval(intervals, queries);
        assert_eq!(result, vec![6, 6, 4, 1, 3, 4, 6, 6, 6, 3]);
    }
}
