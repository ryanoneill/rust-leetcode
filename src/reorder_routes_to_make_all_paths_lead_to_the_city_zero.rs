/// There are `n` cities numbered from `0` to `n - 1` and `n - 1` roads such
/// that there is only one way to travel between two cities (this network form
/// a tree). Last year, the ministry of transport decided to orient the roads
/// in one direction because they are too narrow.
///
/// Roads are represented by `connections` where `connections[i] = [ai, bi]`
/// represents a road from city `ai` to city `bi`.
///
/// This year, there will be a big event in the capital (city `0`) and many
/// people want to travel to this city.
///
/// Your task consists of reorienting some roads such that each city can visit
/// the city `0`. Return the minimum number of edges changed.
///
/// It's guaranteed that each city can reach city `0` after reorder.
struct Solution;

impl Solution {

    // TODO: Implement
    pub fn min_reorder(n: i32, connections: Vec<Vec<i32>>) -> i32 {
        0
    }

}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[ignore]
    #[test]
    fn example_1() {
        let n = 6;
        let connections = vec![vec![0,1], vec![1,3], vec![2,3], vec![4,0], vec![4,5]];
        let result = Solution::min_reorder(n, connections);
        assert_eq!(result, 3);
    }

    #[ignore]
    #[test]
    fn example_2() {
        let n = 5;
        let connections = vec![vec![1,0], vec![1,2], vec![3,2], vec![3,4]];
        let result = Solution::min_reorder(n, connections);
        assert_eq!(result, 2);
    }

    #[ignore]
    #[test]
    fn example_3() {
        let n = 3;
        let connections = vec![vec![1,0], vec![2,0]];
        let result = Solution::min_reorder(n, connections);
        assert_eq!(result, 0);
    }

}
