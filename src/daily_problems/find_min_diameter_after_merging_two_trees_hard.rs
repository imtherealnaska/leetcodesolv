use std::collections::VecDeque;

pub fn minimum_diameter_after_merge(edges1: Vec<Vec<i32>>, edges2: Vec<Vec<i32>>) -> i32 {
    let n = edges1.len() + 1;
    let m = edges2.len() + 1;

    let adj_list1 = build_adj_list(n, &edges1);
    let adj_list2 = build_adj_list(m, &edges2);

    let diameter1 = find_diameter(n, &adj_list1);
    let diameter2 = find_diameter(m, &adj_list2);

    let combined_diameter = (diameter1 + 1) / 2 + (diameter2 + 1) / 2 + 1;
    diameter1.max(diameter2).max(combined_diameter)
}

fn build_adj_list(size: usize, edges: &Vec<Vec<i32>>) -> Vec<Vec<i32>> {
    let mut adj_list = vec![Vec::new(); size];

    for edge in edges {
        let u = edge[0] as usize;
        let v = edge[1] as usize;
        adj_list[u].push(v as i32);
        adj_list[v].push(u as i32);
    }
    adj_list
}

fn find_diameter(n: usize, adjlist: &Vec<Vec<i32>>) -> i32 {
    let mut leaves_queue = VecDeque::new();
    let mut degrees = vec![];

    for node in 0..n {
        degrees[node] = adjlist[node].len();
        if degrees[node] == 1 {
            leaves_queue.push_back(node);
        }
    }

    let mut remaining_nodes = n;
    let mut leaves_layers_removed = 0;

    while remaining_nodes > 2 {
        let size = leaves_queue.len();
        remaining_nodes -= size;
        leaves_layers_removed += 1;

        for _ in 0..size {
            let current_node = leaves_queue.pop_front().unwrap();

            for &neighbour in &adjlist[current_node] {
                let neighbour = neighbour as usize;
                degrees[neighbour] -= 1;
                if degrees[neighbour] == 1 {
                    leaves_queue.push_back(neighbour);
                }
            }
        }
    }
    if remaining_nodes == 2 {
        2 * leaves_layers_removed + 1
    } else {
        2 * leaves_layers_removed
    }
}
