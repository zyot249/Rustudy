/*
Given:
- list airline tickets: ticket[i] = [from i, to i]

Return:
- the itinerary with places in order

Constraints:
- smallest lexical order
- use all tickets once and only once
- start from "JFK"
- tickets len: [1, 300]
- from i, to i length: 3 (uppercase letter)
- from i != to i

*/

use std::cmp::Reverse;
use std::collections::{BinaryHeap, HashMap};

use crate::advanced_graph::graph_struct::graph;

struct Solution;

impl Solution {
    pub fn find_itinerary(tickets: Vec<Vec<String>>) -> Vec<String> {
        // let mut destinations_from: HashMap<String, BinaryHeap<Reverse<String>>> = HashMap::new();

        // for ticket in tickets {
        //     destinations_from
        //         .entry(ticket[0].clone())
        //         .and_modify(|destinations| destinations.push(Reverse(ticket[1].clone())))
        //         .or_insert(BinaryHeap::from([Reverse(ticket[1].clone())]));
        // }

        // let mut result: Vec<String> = Vec::new();

        // fn dfs(
        //     curr: String,
        //     destinations_from: &mut HashMap<String, BinaryHeap<Reverse<String>>>,
        //     result: &mut Vec<String>,
        // ) {
        //     // Process all destinations from current airport
        //     while let Some(dest) = destinations_from.get_mut(&curr).and_then(|dests| dests.pop()) {
        //         dfs(dest.0, destinations_from, result);
        //     }
        //     // Add current airport to result (in reverse order)
        //     result.push(curr);
        // }

        // // Start DFS from JFK
        // dfs("JFK".to_string(), &mut destinations_from, &mut result);

        // // Reverse to get correct order
        // result.reverse();
        // result

        // refined version
        let mut graph: HashMap<&str, BinaryHeap<Reverse<&str>>> =
            HashMap::with_capacity(tickets.len());

        for ticket in &tickets {
            graph
                .entry(&ticket[0])
                .and_modify(|heap| heap.push(Reverse(&ticket[1])))
                .or_insert_with(|| {
                    let mut heap: BinaryHeap<Reverse<&str>> =
                        BinaryHeap::with_capacity(tickets.len());
                    heap.push(Reverse(&ticket[1]));
                    heap
                });
        }

        let mut itinerary: Vec<&str> = Vec::with_capacity(tickets.len() + 1);
        Self::dfs("JFK", &mut graph, &mut itinerary);

        itinerary.into_iter().map(String::from).rev().collect()
    }

    fn dfs<'a>(
        cur_airport: &'a str,
        graph: &mut HashMap<&'a str, BinaryHeap<Reverse<&'a str>>>,
        itinerary: &mut Vec<&'a str>,
    ) {
        while let Some(Reverse(next_airport)) = graph.get_mut(cur_airport).and_then(|dests| dests.pop()) {
            Self::dfs(next_airport, graph, itinerary);
        }

        itinerary.push(cur_airport);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_find_itinerary() {
        // Original test case
        assert_eq!(
            Solution::find_itinerary(vec![
                vec!["JFK".to_string(), "SFO".to_string()],
                vec!["JFK".to_string(), "ATL".to_string()],
                vec!["SFO".to_string(), "ATL".to_string()],
                vec!["ATL".to_string(), "JFK".to_string()],
                vec!["ATL".to_string(), "SFO".to_string()]
            ]),
            vec!["JFK", "ATL", "JFK", "SFO", "ATL", "SFO"]
        );

        // Test case requiring backtracking
        assert_eq!(
            Solution::find_itinerary(vec![
                vec!["JFK".to_string(), "KUL".to_string()],
                vec!["JFK".to_string(), "NRT".to_string()],
                vec!["NRT".to_string(), "JFK".to_string()]
            ]),
            vec!["JFK", "NRT", "JFK", "KUL"]
        );
    }
}
