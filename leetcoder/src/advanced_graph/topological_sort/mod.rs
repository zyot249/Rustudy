/*
    Input graph is represented in form of an adjacency list
*/
fn topological_sort_dfs(graph: Vec<Vec<usize>>) -> Result<Vec<usize>, &'static str> {
    let vertice_count = graph.len();

    let mut visited: Vec<bool> = vec![false; vertice_count];
    let mut stack: Vec<usize> = Vec::new();

    for v in 0..vertice_count {
        dfs(v, &graph, &mut visited, &mut stack);
    }

    stack.reverse();
    Ok(stack)
}

fn dfs(vertex: usize, graph: &Vec<Vec<usize>>, visited: &mut Vec<bool>, stack: &mut Vec<usize>) {
    if visited[vertex] {
        return;
    }

    visited[vertex] = true;

    for &adj in graph[vertex].iter() {
        dfs(adj, graph, visited, stack);
    }

    stack.push(vertex);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_topological_sort_dfs() {
        let graph = vec![vec![1], vec![2], vec![3], vec![]];
        let result = topological_sort_dfs(graph);
        assert_eq!(result, Ok(vec![0, 1, 2, 3]));
    }

    #[test]
    fn test_topological_sort_dfs_2() {
        let graph = vec![vec![], vec![], vec![3], vec![1], vec![0, 1], vec![0, 2]];
        let result = topological_sort_dfs(graph);
        assert_eq!(result, Ok(vec![5, 4, 2, 3, 1, 0]));
    }
}
