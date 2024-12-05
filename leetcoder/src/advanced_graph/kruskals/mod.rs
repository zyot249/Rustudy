use super::graph_struct::{
    disjoint_sets::DisjointSets,
    graph::{Edge, Graph},
};

impl Graph {
    /// Finds the Minimum Spanning Tree using Kruskal's algorithm
    /// Returns Result containing either the MST edges or an error
    pub fn kruskal(&self) -> Result<Vec<Edge>, &'static str> {
        if self.vertices == 0 {
            return Err("Graph must have at least one vertex");
        }

        let mut mst_edges: Vec<Edge> = Vec::new();
        let mut sorted_edges = self.edges.clone();
        sorted_edges.sort_by_key(|edge| edge.weight);

        let mut disjoint_sets = DisjointSets::new(self.vertices);

        // Process edges in ascending order of weight
        for edge in sorted_edges {
            let src_parent = disjoint_sets.find(edge.src);
            let dest_parent = disjoint_sets.find(edge.dest);

            // Add edge if it doesn't create a cycle
            if src_parent != dest_parent {
                disjoint_sets.union(edge.src, edge.dest);
                mst_edges.push(edge);
            }

            // Early exit if we have enough edges for MST
            if mst_edges.len() == self.vertices - 1 {
                break;
            }
        }

        // Check if we found a valid MST
        if mst_edges.len() != self.vertices - 1 {
            return Err("Graph is not connected - no valid MST exists");
        }

        Ok(mst_edges)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_kruskal() {
        let mut graph = Graph::new(4);
        graph.add_edge(0, 1, 10);
        graph.add_edge(0, 2, 6);
        graph.add_edge(0, 3, 5);
        graph.add_edge(1, 3, 15);
        graph.add_edge(2, 3, 4);

        let result = graph.kruskal();
        assert!(result.is_ok());
        let mst_edges = result.unwrap();
        for edge in mst_edges.iter() {
            println!("{} - {} : {}", edge.src, edge.dest, edge.weight);
        }
    }
}
