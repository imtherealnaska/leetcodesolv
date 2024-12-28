use std::collections::BinaryHeap;

#[derive(Debug, PartialEq, Eq)]
struct Node {
    vertex: usize,
    distance: u32,
}

impl Ord for Node {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other.distance.cmp(&self.distance).reverse()
    }
}

impl PartialOrd for Node {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

pub fn djikstra(graph: &[Vec<(usize, u32)>], start: usize) -> Vec<u32> {
    let mut distances = vec![u32::MAX; graph.len()];
    let mut visited = vec![false; graph.len()];
    let mut heap = BinaryHeap::new();

    distances[start] = 0;
    heap.push(Node {
        vertex: start,
        distance: 0,
    });

    while let Some(Node { vertex, distance }) = heap.pop() {
        if visited[vertex] {
            continue;
        }

        visited[vertex] = true;

        for &(neighbour, weight) in &graph[vertex] {
            let new_distance = distance + weight;

            if new_distance < distances[neighbour] {
                distances[neighbour] = new_distance;
                heap.push(Node {
                    vertex: neighbour,
                    distance: new_distance,
                });
            }
        }
    }
    distances
}

#[test]
fn djikstra_tes() {
    let graph = vec![
        vec![(1, 4), (2, 1)],
        vec![(3, 1)],
        vec![(1, 2), (3, 5)],
        vec![],
    ];

    let distances = djikstra(&graph, 0);
    println!("Shortest distances from node 0: {:?}", distances);
}
