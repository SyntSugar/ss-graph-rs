use std::collections::HashMap;
use std::collections::HashSet;
use std::hash::Hash;

#[derive(Clone, PartialEq, Eq)]
pub struct Graph<T: Eq + Hash + Clone> {
    is_directed: bool,
    adjacency_list: HashMap<T, HashSet<T>>,
}

impl<T> Graph<T>
where
    T: Eq + Hash + Clone,
{
    pub fn new(is_directed: Option<bool>) -> Self {
        let is_directed = is_directed.unwrap_or(false); // default to undirected graph
        Graph {
            is_directed,
            adjacency_list: HashMap::new(),
        }
    }

    // Add a node to the graph.
    pub fn add_edge(&mut self, vector_x: T, vector_y: T) {
        self.adjacency_list
            .entry(vector_x.clone())
            .or_insert(HashSet::new())
            .insert(vector_y.clone());

        if !self.is_directed {
            self.adjacency_list
                .entry(vector_y.clone())
                .or_insert(HashSet::new())
                .insert(vector_x.clone());
        }
    }

    // Use depth-first search to find all paths between two nodes
    pub fn find_all_paths(&self, start: T, end: T) -> Vec<Vec<T>> {
        let mut paths = Vec::new();
        let mut visited = HashSet::new();
        let mut path = Vec::new();

        path.push(start.clone());
        visited.insert(start.clone());

        self.find_all_paths_helper(start, end, &mut visited, &mut path, &mut paths);

        paths
    }

    // Helper function for find_all_paths. This function is recursive.
    fn find_all_paths_helper(
        &self,
        start: T,
        end: T,
        visited: &mut HashSet<T>,
        path: &mut Vec<T>,
        paths: &mut Vec<Vec<T>>,
    ) {
        if start == end {
            paths.push(path.clone());
            return;
        }

        if let Some(neighbors) = self.adjacency_list.get(&start) {
            for neighbor in neighbors {
                if !visited.contains(neighbor) {
                    visited.insert(neighbor.clone());
                    path.push(neighbor.clone());
                    self.find_all_paths_helper(neighbor.clone(), end.clone(), visited, path, paths);
                    path.pop();
                    visited.remove(neighbor);
                }
            }
        }
    }

    // Use depth-first search to find all paths between two nodes with max steps limit.
    pub fn find_paths_with_max_steps(&self, start: T, end: T, max_steps: usize) -> Vec<Vec<T>> {
        let mut paths = Vec::new();
        let mut visited = HashSet::new();
        let mut path = Vec::new();

        path.push(start.clone());
        visited.insert(start.clone());

        self.find_paths_with_max_steps_helper(
            start,
            end.clone(),
            max_steps,
            &mut visited,
            &mut path,
            &mut paths,
        );

        paths
    }

    // Helper function for find_paths_with_max_steps. This function is recursive.
    fn find_paths_with_max_steps_helper(
        &self,
        start: T,
        end: T,
        max_steps: usize,
        visited: &mut HashSet<T>,
        path: &mut Vec<T>,
        paths: &mut Vec<Vec<T>>,
    ) {
        if start == end {
            paths.push(path.clone());
            return;
        }

        if path.len() >= max_steps {
            return;
        }

        if let Some(neighbors) = self.adjacency_list.get(&start) {
            for neighbor in neighbors {
                if !visited.contains(neighbor) {
                    visited.insert(neighbor.clone());
                    path.push(neighbor.clone());
                    self.find_paths_with_max_steps_helper(
                        neighbor.clone(),
                        end.clone(),
                        max_steps,
                        visited,
                        path,
                        paths,
                    );
                    path.pop();
                    visited.remove(neighbor);
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // Test the Graph struct with integers.
    #[test]
    fn test_add_edge() {
        let mut graph = Graph::new(Some(false));
        graph.add_edge(1, 2);
        assert!(graph.adjacency_list.get(&1).unwrap().contains(&2));
        assert!(graph.adjacency_list.get(&2).is_some());

        let mut graph = Graph::new(Some(true));
        graph.add_edge(1, 2);
        assert!(graph.adjacency_list.get(&1).unwrap().contains(&2));
        assert!(!graph.adjacency_list.get(&2).is_some());
    }

    #[test]
    fn test_find_all_paths() {
        let mut graph = Graph::new(Some(false));
        graph.add_edge(1, 2);
        graph.add_edge(2, 3);
        graph.add_edge(3, 4);
        let paths = graph.find_all_paths(1, 4);
        assert_eq!(paths, vec![vec![1, 2, 3, 4]]);
    }

    #[test]
    fn test_find_paths_with_max_steps() {
        let mut graph = Graph::new(None);
        graph.add_edge(1, 2);
        graph.add_edge(2, 3);
        graph.add_edge(3, 4);
        let paths = graph.find_paths_with_max_steps(1, 4, 3);
        assert_eq!(paths, Vec::<Vec<i64>>::new());
        let paths = graph.find_paths_with_max_steps(1, 4, 4);
        assert_eq!(paths, vec![vec![1, 2, 3, 4]]);
    }

    // Test the Graph struct with strings.
    #[test]
    fn test_add_edge_string() {
        let mut graph = Graph::new(Some(false));
        graph.add_edge("Node1".to_string(), "Node2".to_string());
        assert!(graph
            .adjacency_list
            .get(&"Node1".to_string())
            .unwrap()
            .contains(&"Node2".to_string()));
        assert!(graph
            .adjacency_list
            .get(&"Node2".to_string())
            .unwrap()
            .contains(&"Node1".to_string()));

        let mut graph = Graph::new(Some(true));
        graph.add_edge("Node1".to_string(), "Node2".to_string());
        assert!(graph
            .adjacency_list
            .get(&"Node1".to_string())
            .unwrap()
            .contains(&"Node2".to_string()));
        assert!(!graph.adjacency_list.get(&"Node2".to_string()).is_some());
    }

    #[test]
    fn test_find_all_paths_string() {
        let mut graph = Graph::new(Some(false));
        graph.add_edge("Node1".to_string(), "Node2".to_string());
        graph.add_edge("Node2".to_string(), "Node3".to_string());
        graph.add_edge("Node3".to_string(), "Node4".to_string());
        let paths = graph.find_all_paths("Node1".to_string(), "Node4".to_string());
        assert_eq!(
            paths,
            vec![vec!["Node1", "Node2", "Node3", "Node4"]
                .into_iter()
                .map(|s| s.to_string())
                .collect::<Vec<_>>()]
        );
    }

    #[test]
    fn test_find_paths_with_max_steps_string() {
        let mut graph = Graph::new(Some(false));
        graph.add_edge("Node1".to_string(), "Node2".to_string());
        graph.add_edge("Node2".to_string(), "Node3".to_string());
        graph.add_edge("Node3".to_string(), "Node4".to_string());
        let paths = graph.find_paths_with_max_steps("Node1".to_string(), "Node4".to_string(), 3);
        assert_eq!(paths, Vec::<Vec<String>>::new());
        let paths = graph.find_paths_with_max_steps("Node1".to_string(), "Node4".to_string(), 4);
        assert_eq!(
            paths,
            vec![vec!["Node1", "Node2", "Node3", "Node4"]
                .into_iter()
                .map(|s| s.to_string())
                .collect::<Vec<_>>()]
        );
    }
}
