use std::collections::HashMap;
use std::collections::HashSet;

#[derive(Clone, Debug, PartialEq)]
pub struct AdjacencyGraph {
    n: usize,
    neighbors: HashMap<i32, HashSet<i32>>,
}

impl AdjacencyGraph {

    pub fn new(n: usize) -> Self {
        Self { n, neighbors: HashMap::new() }
    }

    pub fn from_neighbors(n: usize, neighbors: &HashMap<i32, HashSet<i32>>) -> Self {
        Self { n, neighbors: neighbors.clone() }
    }

    pub fn from_vec(n: usize, edges: &Vec<Vec<i32>>) -> Self {
        let mut result = Self::new(n);
        for edge in edges {
            result.add_edge(edge[0], edge[1]);
        }
        result
    }

    pub fn add_edge(&mut self, x: i32, y: i32) {
        self.neighbors
            .entry(x)
            .or_insert(HashSet::new())
            .insert(y);
        self.neighbors
            .entry(y)
            .or_insert(HashSet::new())
            .insert(x);
    }

    pub fn remove_edge(&mut self, x: i32, y: i32) {
        self.neighbors
            .entry(x)
            .and_modify(|s| { s.remove(&y); });
        self.neighbors
            .entry(y)
            .and_modify(|s| { s.remove(&x); });
    }

    pub fn is_leaf(&self, x: i32) -> bool {
        self.neighbors
            .get(&x)
            .map(|s| s.len() == 1)
            .unwrap_or_default()
    }

    pub fn find_leaves(&self) -> Vec<i32> {
        self.neighbors
            .iter()
            .filter(|(_, v)| v.len() == 1)
            .map(|(k, _)| k)
            .copied()
            .collect()
    }

    pub fn find_leaf_neighbor(&self, x: i32) -> i32 {
        self.neighbors
            .get(&x)
            .and_then(|s| s.iter().next())
            .copied()
            .unwrap_or(-1)
    }

    pub fn find_centroids(&self) -> Vec<i32> {
        if self.neighbors.len() == 0 {
            (0..self.n).into_iter().map(|i| i as i32).collect()
        } else if self.neighbors.len() <= 2 {
            self.neighbors.keys().copied().collect()
        } else {
            let mut cloned = self.clone();

            let mut leaves = cloned.find_leaves();
            let mut left = cloned.neighbors.len();
            while left > 2 {
                let mut next_leaves = Vec::new();
                left -= leaves.len();
                for &leaf in &leaves {
                    let neighbor = cloned.find_leaf_neighbor(leaf);
                    cloned.remove_edge(leaf, neighbor);
                    if cloned.is_leaf(neighbor) {
                        next_leaves.push(neighbor);
                    }
                }
                leaves = next_leaves;
            }

            leaves
        }
    }

    pub fn vertices_len(&self) -> usize {
        self.n
    }

    pub fn edges_len(&self) -> usize {
        self.neighbors.len()
    }

    pub fn adj(&self, x: i32) -> HashSet<i32> {
        self.neighbors
            .get(&x)
            .cloned()
            .unwrap_or(HashSet::new())
    }

}
