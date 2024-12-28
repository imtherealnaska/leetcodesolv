pub struct UnionFInd {
    parent: Vec<usize>,
    rank: Vec<usize>,
}

impl UnionFInd {
    pub fn new(size: usize) -> Self {
        UnionFInd {
            parent: (0..size).collect(),
            rank: vec![0; size],
        }
    }

    pub fn find(&mut self, mut x: usize) -> usize {
        let mut root = x;

        // Find the root
        while self.parent[root] != root {
            root = self.parent[root];
        }

        // Path compression - point all nodes to root
        while x != root {
            let next = self.parent[x];
            self.parent[x] = root;
            x = next;
        }

        root
    }

    pub fn union(&mut self, x: usize, y: usize) {
        let root_x = self.find(x);
        let roo_y = self.find(y);

        if root_x != roo_y {
            match self.rank[root_x].cmp(&self.rank[roo_y]) {
                std::cmp::Ordering::Less => {
                    self.parent[root_x] = roo_y;
                }
                std::cmp::Ordering::Equal => {
                    self.parent[roo_y] = root_x;
                    self.parent[root_x] += 1;
                }
                std::cmp::Ordering::Greater => {
                    self.parent[roo_y] = root_x;
                }
            }
        }
    }
}
