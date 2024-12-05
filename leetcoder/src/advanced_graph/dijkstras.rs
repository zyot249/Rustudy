use std::cmp::Reverse;
use std::collections::BinaryHeap;

fn dijkstras(graph: Vec<Vec<i32>>, start: i32) -> Vec<i32> {
    let mut distances = vec![i32::MAX; graph.len()];
    distances[start as usize] = 0;
    let mut pq = BinaryHeap::new();
    pq.push(Reverse((0, start)));

    while let Some(Reverse((base_distance, vertex))) = pq.pop() {
        let adjacents = &graph[vertex as usize];
        for neighbor in 0..adjacents.len() {
            if adjacents[neighbor] == 0 {
                continue;
            }

            let distance = base_distance + adjacents[neighbor];
            if distance < distances[neighbor] {
                distances[neighbor] = distance;
                pq.push(Reverse((distance, neighbor as i32)));
            }
        }
    }

    distances
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_dijkstras() {
        let graph = vec![
            vec![0, 4, 0, 0, 0, 0, 0, 8, 0],
            vec![4, 0, 8, 0, 0, 0, 0, 11, 0],
            vec![0, 8, 0, 7, 0, 4, 0, 0, 2],
            vec![0, 0, 7, 0, 9, 14, 0, 0, 0],
            vec![0, 0, 0, 9, 0, 10, 0, 0, 0],
            vec![0, 0, 4, 14, 10, 0, 2, 0, 0],
            vec![0, 0, 0, 0, 0, 2, 0, 1, 6],
            vec![8, 11, 0, 0, 0, 0, 1, 0, 7],
            vec![0, 0, 2, 0, 0, 0, 6, 7, 0],
        ];
        let distances = dijkstras(graph, 0);
        assert_eq!(distances, vec![0, 4, 12, 19, 21, 11, 9, 8, 14]);
    }
}
