struct UnionFind {
    root: Vec<i32>,
    rank: Vec<i32>,
}

impl UnionFind {

    /// O(n)
    fn new(n: usize) -> Self {
        let mut root = vec![0; n];
        let mut rank = vec![0; n];
        for i in 0..n {
            root[i] = i as i32;
            rank[i] = i as i32;
        }
        Self { root, rank }
    }

    /// O(log n)
    pub fn find(&self, x: i32) -> i32 {
        let mut x = x;
        while x != self.root[x as usize] {
            x = self.root[x as usize];
        }
        x
    }

    /// O(log n)
    pub fn union(&mut self, x: i32, y: i32) {
        let root_x = self.find(x);
        let root_y = self.find(y);
        if root_x != root_y {
            if self.rank[root_x as usize] > self.rank[root_y as usize] {
                self.root[root_y as usize] = root_x;
            } else if self.rank[root_x as usize] < self.rank[root_y as usize] {
                self.root[root_x as usize] = root_y;
            } else {
                self.root[root_y as usize] = root_x;
                self.rank[root_x as usize] += 1;
            }
        }
    }

    /// O(log n)
    pub fn connected(&self, x: i32, y: i32) -> bool {
        self.find(x) == self.find(y)
    }

}

#[cfg(test)]
mod tests {
    use super::UnionFind;

    #[test]
    fn run_through() {
        let mut uf = UnionFind::new(10);
        uf.union(1, 2);
        uf.union(2, 5);
        uf.union(5, 6);
        uf.union(6, 7);
        uf.union(3, 8);
        uf.union(8, 9);
        assert!(uf.connected(1, 5));
        assert!(uf.connected(5, 7));
        assert!(!uf.connected(4, 9));
        uf.union(9, 4);
        assert!(uf.connected(4, 9));
    }

}
