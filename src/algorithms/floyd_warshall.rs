pub fn floyd_warshall(
    graph: &Vec<Vec<Option<i32>>>,
) -> (Vec<Vec<Option<i32>>>, Vec<Vec<Option<usize>>>) {
    let n = graph.len();
    let mut dist = graph.clone();

    let mut next = vec![vec![None; n]; n];
    for i in 0..n {
        for j in 0..n {
            if graph[i][j].is_some() {
                next[i][j] = Some(j);
            }
        }
    }

    for k in 0..n {
        for i in 0..n {
            for j in 0..n {
                match (dist[i][k], dist[k][j]) {
                    (Some(ik), Some(kj)) => {
                        let through_k = ik.checked_add(kj);
                        match (through_k, dist[i][j]) {
                            (Some(new_dist), Some(current)) if new_dist < current => {
                                dist[i][j] = Some(new_dist);
                                next[i][j] = next[i][k];
                            }
                            (Some(new_dist), None) => {
                                dist[i][j] = Some(new_dist);
                                next[i][j] = next[i][k];
                            }
                            _ => {}
                        }
                    }
                    _ => {}
                }
            }
        }
    }
    (dist, next)
}

pub fn reconstruct_path(
    next: &Vec<Vec<Option<usize>>>,
    start: usize,
    end: usize,
) -> Option<Vec<usize>> {
    if next[start][end].is_none() {
        return None;
    }

    let mut path = vec![start];
    let mut current = start;

    while current != end {
        current = next[current][end].unwrap();
        path.push(current);
    }
    Some(path)
}

#[test]
fn floyd_works() {
    let graph = vec![
        vec![Some(0), Some(5), None, Some(10)],
        vec![None, Some(0), Some(3), None],
        vec![None, None, Some(0), Some(1)],
        vec![None, None, None, Some(0)],
    ];

    let (distance, next) = floyd_warshall(&graph);

    println!("Shortest distance matrix:");
    for row in &distance {
        println!("{:?}", row);
    }

    if let Some(path) = reconstruct_path(&next, 0, 3) {
        println!("Shortest path from 0 to 3 : {:?}", path);
    }
}
