use std::collections::HashSet;

pub struct DFSIterator<'a, T> {
    graph: &'a Vec<Vec<usize>>,
    stack: Vec<usize>,
    visited: HashSet<usize>,
    value_map: &'a Vec<T>,
}

impl<'a, T> DFSIterator<'a, T> {
    pub fn new(graph: &'a Vec<Vec<usize>>, start: usize, value_map: &'a Vec<T>) -> Self {
        let mut visited = HashSet::new();
        visited.insert(start);

        DFSIterator {
            graph,
            stack: vec![start],
            visited,
            value_map,
        }
    }
}

impl<'a, T: Clone> Iterator for DFSIterator<'a, T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(current) = self.stack.pop() {
            for &neighbour in self.graph[current].iter().rev() {
                if !self.visited.contains(&neighbour) {
                    self.visited.insert(neighbour);
                    self.stack.push(neighbour);
                }
            }
            Some(self.value_map[current].clone())
        } else {
            None
        }
    }
}

#[test]
fn dfs_works() {
    let graph = vec![vec![1, 2], vec![3], vec![3], vec![]];

    let values = vec!["A", "B", "C", "D"];
    let dfs = DFSIterator::new(&graph, 0, &values);

    for value in dfs {
        println!("{}", value);
    }
}
