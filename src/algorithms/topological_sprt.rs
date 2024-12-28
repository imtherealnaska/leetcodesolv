use std::{
    collections::{HashMap, VecDeque},
    usize,
};

#[derive(Debug)]
pub struct Graph {
    graph: HashMap<usize, Vec<usize>>,
    vertices: usize,
}

impl Graph {
    fn new() -> Self {
        Graph {
            graph: HashMap::new(),
            vertices: 0,
        }
    }

    fn add_edge(&mut self, u: usize, v: usize) {
        self.graph.entry(u).or_insert(Vec::new()).push(v);
        self.vertices = self.vertices.max(u + 1).max(v + 1);
    }

    fn topological_sort_kahn(&self) -> Result<Vec<usize>, &'static str> {
        let mut indegree = vec![0; self.vertices];

        for (u, neighbours) in &self.graph {
            for &v in neighbours {
                indegree[v] += 1;
            }
        }

        let mut queue = VecDeque::new();
        for u in 0..self.vertices {
            if indegree[u] == 0 {
                queue.push_back(u);
            }
        }

        let mut result = Vec::new();
        while let Some(u) = queue.pop_front() {
            result.push(u);

            if let Some(neighbours) = self.graph.get(&u) {
                for &v in neighbours {
                    indegree[v] -= 1;
                    if indegree[v] == 0 {
                        queue.push_back(v);
                    }
                }
            }
        }

        if result.len() != self.vertices {
            return Err("Graph contains a cycle");
        }
        Ok(result)
    }
}
