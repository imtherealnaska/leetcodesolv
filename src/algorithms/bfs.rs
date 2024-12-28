use std::{
    collections::{HashMap, HashSet, VecDeque},
    hash::Hash,
};

#[derive(Default)]
pub struct Graph<T>
where
    T: Eq + Hash + Clone,
{
    adj_list: HashMap<T, Vec<T>>,
}

impl<T> Graph<T>
where
    T: Eq + Hash + Clone,
{
    pub fn new() -> Self {
        Graph {
            adj_list: HashMap::new(),
        }
    }

    pub fn add_node(&mut self, node: T) {
        self.adj_list.entry(node).or_default();
    }

    pub fn add_edge(&mut self, from: T, to: T) {
        self.adj_list
            .entry(from.clone())
            .or_default()
            .push(to.clone());
    }

    pub fn bfs(&self, start: T) -> Vec<T> {
        let mut visited = HashSet::new();
        let mut queue = VecDeque::new();
        let mut result = Vec::new();

        queue.push_back(start.clone());
        visited.insert(start);

        while let Some(current) = queue.pop_front() {
            result.push(current.clone());

            if let Some(neighbours) = self.adj_list.get(&current) {
                for neighbour in neighbours {
                    if !visited.contains(neighbour) {
                        queue.push_back(neighbour.clone());
                        visited.insert(neighbour.clone());
                    }
                }
            }
        }
        result
    }

    pub fn shortest_path(&self, start: T, end: T) -> Option<(Vec<T>, usize)> {
        let mut visited = HashSet::new();
        let mut queue = VecDeque::new();
        let mut parent: HashMap<T, T> = HashMap::new();

        queue.push_back(start.clone());
        visited.insert(start);

        while let Some(current) = queue.pop_front() {
            if current == end {
                let mut path = Vec::new();
                let mut current = current;

                while let Some(p) = parent.get(&current) {
                    path.push(current.clone());
                    current = p.clone();
                }
                path.push(current);
                path.reverse();
                return Some((path.clone(), path.len() - 1));
            }

            if let Some(neighbours) = self.adj_list.get(&current) {
                for neigh in neighbours {
                    if !visited.contains(neigh) {
                        queue.push_back(neigh.clone());
                        visited.insert(neigh.clone());
                        parent.insert(neigh.clone(), current.clone());
                    }
                }
            }
        }
        None
    }

    pub fn nodes_at_specific_distance(&self, start: T, distacne: usize) -> Vec<T> {
        let mut visited = HashSet::new();
        let mut queue = VecDeque::new();
        let mut distances = HashMap::new();
        let mut result = Vec::new();

        queue.push_back(start.clone());
        visited.insert(start.clone());
        distances.insert(start, 0);

        while let Some(current) = queue.pop_front() {
            let current_distance = *distances.get(&current).unwrap();
            if current_distance == distacne {
                result.push(current.clone());
                continue;
            }

            if current_distance > distacne {
                break;
            }

            if let Some(neighbours) = self.adj_list.get(&current) {
                for neighbour in neighbours {
                    if !visited.contains(neighbour) {
                        queue.push_back(neighbour.clone());
                        visited.insert(neighbour.clone());
                        distances.insert(neighbour.clone(), current_distance + 1);
                    }
                }
            }
        }
        result
    }
}

#[test]
fn bfs_works() {
    let mut graph = Graph::new();

    // Add nodes and edges
    for i in 0..6 {
        graph.add_node(i);
    }

    graph.add_edge(0, 1);
    graph.add_edge(0, 2);
    graph.add_edge(1, 3);
    graph.add_edge(2, 3);
    graph.add_edge(3, 4);
    graph.add_edge(3, 5);

    // Perform BFS starting from node 0
    println!("BFS traversal starting from 0: {:?}", graph.bfs(0));

    // Find shortest path from 0 to 5
    if let Some((path, length)) = graph.shortest_path(0, 5) {
        println!("Shortest path from 0 to 5: {:?}", path);
        println!("Path length: {}", length);
    }

    // Find all nodes at distance 2 from start node 0
    println!(
        "Nodes at distance 2 from 0: {:?}",
        graph.nodes_at_specific_distance(0, 2)
    );
}
