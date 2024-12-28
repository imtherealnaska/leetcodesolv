use std::collections::{BinaryHeap, HashMap};

#[derive(Debug)]
pub struct Edge {
    to: usize,
    weight: i32,
}

#[derive(Debug, PartialEq, Eq)]
struct State {
    cost: i32,
    vertex: usize,
    parent: Option<usize>,
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other
            .cost
            .cmp(&self.cost)
            .then_with(|| self.vertex.cmp(&other.vertex))
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

pub fn prims_mst(
    graph: &HashMap<usize, Vec<Edge>>,
    start: usize,
) -> Option<(i32, Vec<(usize, usize)>)> {
    let mut total_cost = 0;
    let mut mst_edges = Vec::new();
    let mut visited = vec![false; graph.len()];
    let mut pq = BinaryHeap::new();

    pq.push(State {
        cost: 0,
        vertex: start,
        parent: None,
    });

    while let Some(State {
        cost,
        vertex,
        parent,
    }) = pq.pop()
    {
        if visited[vertex] {
            continue;
        }

        visited[vertex] = true;
        total_cost += cost;

        if let Some(parent_vertex) = parent {
            mst_edges.push((parent_vertex, vertex));
        }

        if let Some(edges) = graph.get(&vertex) {
            for edge in edges {
                pq.push(State {
                    cost: edge.weight,
                    vertex: edge.to,
                    parent: Some(vertex),
                });
            }
        }
    }

    if visited.iter().all(|&v| v) {
        Some((total_cost, mst_edges))
    } else {
        None
    }
}
