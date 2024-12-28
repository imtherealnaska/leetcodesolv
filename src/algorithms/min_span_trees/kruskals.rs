#[derive(Debug, Clone)]
pub struct Edge {
    src: usize,
    dest: usize,
    weight: usize,
}

#[derive(Debug)]
pub struct Graph {
    vertices: usize,
    edges: Vec<Edge>,
}

trait NewTrait {
    fn new(vertices: usize) -> Self;

    fn add_edge(&mut self, src: usize, dest: usize, weight: usize);

    fn kruskal_mst(&self) -> Vec<Edge>;

    fn find(parent: &mut Vec<usize>, node: usize) -> usize;

    fn union(parent: &mut Vec<usize>, rank: &mut Vec<usize>, x: usize, y: usize);
}

impl NewTrait for Graph {
    fn new(vertices: usize) -> Self {
        Graph {
            vertices,
            edges: Vec::new(),
        }
    }

    fn add_edge(&mut self, src: usize, dest: usize, weight: usize) {
        self.edges.push(Edge { src, dest, weight });
    }

    fn kruskal_mst(&self) -> Vec<Edge> {
        let mut edges = self.edges.clone();
        edges.sort_by(|a, b| a.weight.cmp(&b.weight));

        let mut parent = vec![0; self.vertices];
        let mut rank = vec![0; self.vertices];

        // Initialize disjoint set
        for i in 0..self.vertices {
            parent[i] = i;
        }

        let mut mst = Vec::new();

        for edge in edges {
            let root_src = Self::find(&mut parent, edge.src);
            let root_dest = Self::find(&mut parent, edge.dest);

            if root_src != root_dest {
                mst.push(edge.clone());
                Self::union(&mut parent, &mut rank, root_src, root_dest);
            }

            if mst.len() == self.vertices - 1 {
                break;
            }
        }

        mst
    }

    fn find(parent: &mut Vec<usize>, node: usize) -> usize {
        if parent[node] != node {
            parent[node] = Self::find(parent, parent[node]); // Path compression
        }
        parent[node]
    }

    fn union(parent: &mut Vec<usize>, rank: &mut Vec<usize>, x: usize, y: usize) {
        let root_x = Self::find(parent, x);
        let root_y = Self::find(parent, y);

        if root_x != root_y {
            match rank[root_x].cmp(&rank[root_y]) {
                std::cmp::Ordering::Less => parent[root_x] = root_y,
                std::cmp::Ordering::Greater => parent[root_y] = root_x,
                std::cmp::Ordering::Equal => {
                    parent[root_y] = root_x;
                    rank[root_x] += 1;
                }
            }
        }
    }
}

#[test]
fn krusk() {
    let mut graph = Graph::new(4);
    graph.add_edge(0, 1, 10);
    graph.add_edge(0, 2, 6);
    graph.add_edge(0, 3, 5);
    graph.add_edge(1, 3, 15);
    graph.add_edge(2, 3, 4);

    let mst = graph.kruskal_mst();
    println!("Edges in the Minimum Spanning Tree:");
    for edge in mst {
        println!("{} -- {} == {}", edge.src, edge.dest, edge.weight);
    }
}
