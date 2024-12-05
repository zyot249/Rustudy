use std::collections::BinaryHeap;
use std::cmp::Ordering;

/// Represents an edge in the graph with its weight and destination
#[derive(Copy, Clone, Eq, PartialEq)]
struct Edge {
    dest: usize,
    weight: i32,
}

/// Custom ordering for Edge to create a min-heap
impl Ord for Edge {
    fn cmp(&self, other: &Self) -> Ordering {
        other.weight.cmp(&self.weight)
            .then_with(|| self.dest.cmp(&other.dest))
    }
}

impl PartialOrd for Edge {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

/// Finds the Minimum Spanning Tree using Prim's algorithm
/// Returns Result containing either the parent array or an error
fn prims_mst(graph: Vec<Vec<i32>>) -> Result<Vec<i32>, &'static str> {
    let vertex_count = graph.len();
    if vertex_count == 0 {
        return Err("Graph must have at least one vertex");
    }

    let mut parent = vec![-1; vertex_count];
    let mut visited = vec![false; vertex_count];
    let mut heap = BinaryHeap::new();

    // Start from vertex 0
    heap.push(Edge { dest: 0, weight: 0 });

    while let Some(Edge { dest: u, weight: _}) = heap.pop() {
        if visited[u] {
            continue;
        }
        visited[u] = true;

        // Add all adjacent vertices to the priority queue
        for v in 0..vertex_count {
            let weight = graph[u][v];
            if weight > 0 && !visited[v] {
                heap.push(Edge { dest: v, weight: -weight }); // Negative for min-heap
                parent[v] = u as i32;
            }
        }
    }

    // Check if MST includes all vertices
    if !visited.iter().all(|&v| v) {
        return Err("Graph is not connected - no valid MST exists");
    }

    Ok(parent)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_prims_mst() {
        let graph = vec![
            vec![0, 2, 0, 6, 0],
            vec![2, 0, 3, 8, 5],
            vec![0, 3, 0, 0, 7],
            vec![6, 8, 0, 0, 9],
            vec![0, 5, 7, 9, 0],
        ];
        let result = prims_mst(graph);
        assert_eq!(result, Ok(vec![-1, 0, 1, 0, 1]));
    }

    #[test]
    fn test_empty_graph() {
        let graph: Vec<Vec<i32>> = vec![];
        let result = prims_mst(graph);
        assert!(result.is_err());
    }

    #[test]
    fn test_disconnected_graph() {
        let graph = vec![
            vec![0, 0, 0],
            vec![0, 0, 0],
            vec![0, 0, 0],
        ];
        let result = prims_mst(graph);
        assert!(result.is_err());
    }
}
