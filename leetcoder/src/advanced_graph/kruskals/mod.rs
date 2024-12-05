mod disjoint_sets;
mod graph;

use graph::{Edge, Graph};

fn kruskal(graph :Graph) {
    let mut result :Vec<Edge> = vec![];

    let mut edges = graph.edges.clone();
    edges.sort_by_key(|a| a.weight);

    let v = graph.vertices;
    let mut idx :usize = 0;
    while result.len() < v - 1 {
        let next_edge = edges[idx];
        idx += 1;
        result.push(next_edge);

        let g = Graph::new_with_edges(v, &result);
        if g.is_cycle() {
            result.pop();
        }
    }

    for edge in result.iter() {
        println!("{} - {} : {}", edge.src, edge.dest, edge.weight);
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

        kruskal(graph);
    }
}
