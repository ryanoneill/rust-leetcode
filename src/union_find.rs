use std::collections::HashMap;
use std::collections::HashSet;

pub struct UnionFind {
    n: usize,
    root: Vec<i32>,
    rank: Vec<i32>,
    count: usize,
}

impl UnionFind {

    pub fn new(n: usize) -> Self {
        let mut root = vec![0; n];
        let rank = vec![1; n];
        for i in 0..n {
            root[i] = i as i32;
        }
        Self { n, root, rank, count: n }
    }

    pub fn find(&mut self, x: i32) -> i32 {
        if x == self.root[x as usize] { x }
        else {
            self.root[x as usize] = self.find(self.root[x as usize]);
            self.root[x as usize]
        }
    }

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
            self.count -= 1;
        }
    }

    pub fn connected(&mut self, x: i32, y: i32) -> bool {
        self.find(x) == self.find(y)
    }

    pub fn count(&self) -> usize {
        self.count
    }

    pub fn components_by_root(&mut self) -> HashMap<i32, HashSet<i32>> {
        let mut result = HashMap::new();
        for i in 0..self.n {
            let x = i as i32;
            let root = self.find(x);
            result
                .entry(root)
                .or_insert(HashSet::new())
                .insert(x);
        }

        result
    }

    pub fn components(&mut self) -> Vec<Vec<i32>> {
        let by_root = self.components_by_root();
        by_root
            .into_values()
            .map(|s| s.into_iter().collect())
            .collect()
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
