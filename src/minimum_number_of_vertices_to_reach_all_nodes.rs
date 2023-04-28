struct Solution;

impl Solution {

    pub fn find_smallest_set_of_vertices(n: i32, edges: Vec<Vec<i32>>) -> Vec<i32> {
        let n = n as usize;
        let mut reachable = vec![false; n as usize];
        for edge in edges {
            let to = edge[1];
            reachable[to as usize] = true;
        }

        let mut result = Vec::new();
        for i in 0..n {
            if !reachable[i] {
                result.push(i as i32);
            }
        }
        result
    }

}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn example_1() {
        let n = 6;
        let edges = vec![vec![0,1],vec![0,2],vec![2,5],vec![3,4],vec![4,2]];
        let result = Solution::find_smallest_set_of_vertices(n, edges);
        assert_eq!(result, vec![0,3]);
    }

    #[test]
    fn example_2() {
        let n = 5;
        let edges = vec![vec![0,1],vec![2,1],vec![3,1],vec![1,4],vec![2,4]];
        let result = Solution::find_smallest_set_of_vertices(n, edges);
        assert_eq!(result, vec![0,2,3]);
    }

}
