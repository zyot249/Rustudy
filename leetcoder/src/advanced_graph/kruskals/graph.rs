use super::disjoint_sets::DisjointSets;
use std::fmt::Display;

pub struct Graph {
    pub vertices: usize,
    pub edges: Vec<Edge>,
}

#[derive(Clone, Copy)]
pub struct Edge {
    pub src: usize,
    pub dest: usize,
    pub weight: usize,
}

impl Graph {
    pub fn new(vertices: usize) -> Self {
        Graph {
            vertices,
            edges: vec![],
        }
    }

    pub fn new_with_edges(vertices: usize, edges: &Vec<Edge>) -> Self {
        Graph {
            vertices,
            edges: edges.clone(),
        }
    }

    pub fn add_edge(&mut self, src: usize, dest: usize, weight: usize) {
        self.edges.push(Edge { src, dest, weight });
    }

    pub fn is_cycle(&self) -> bool {
        let mut sets = DisjointSets::new(self.vertices);

        for edge in self.edges.iter() {
            if sets.find(edge.src) == sets.find(edge.dest) {
                return true;
            }

            sets.union(edge.src, edge.dest);
        }

        false
    }
}

impl Display for Graph {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Graph with {} vertices and {} edges",
            self.vertices,
            self.edges.len()
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_graph_display() {
        let mut graph = Graph::new(5);
        graph.add_edge(0, 1, 10);
        graph.add_edge(0, 2, 6);
        assert_eq!(format!("{}", graph), "Graph with 5 vertices and 2 edges");
    }

    #[test]
    fn test_is_cycle() {
        let mut graph = Graph::new(3);
        graph.add_edge(0, 1, 1);
        graph.add_edge(1, 2, 1);
        graph.add_edge(0, 2, 1);
        assert_eq!(graph.is_cycle(), true);
    }
}
