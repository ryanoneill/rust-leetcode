use std::collections::HashSet;

/// You are given the array `paths`, where `paths[i] = [cityAi, cityBi]` means there exists a
/// direct path going from `cityAi` to `cityBi`. Return the destination city, that is, the city
/// without any path outgoing to another city.
///
/// It is guaranteed that the graph of paths forms a line without any loop, therefore, there will
/// be exactly one destination city.
struct Solution;

impl Solution {

    pub fn dest_city(paths: Vec<Vec<String>>) -> String {
        let n = paths.len();
        let mut origins = HashSet::with_capacity(n);
        let mut dests = HashSet::with_capacity(n);

        for path in paths {
            origins.insert(path[0].clone());
            dests.insert(path[1].clone());
        }

        let mut result = String::from("");
        for dest in dests {
            if !origins.contains(&dest) {
                result = dest;
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
        let paths = vec![
            vec![str!("London"), str!("New York")],
            vec![str!("New York"), str!("Lima")],
            vec![str!("Lima"), str!("Sao Paulo")]
        ];
        let result = Solution::dest_city(paths);
        assert_eq!(result, "Sao Paulo");
    }

    #[test]
    fn example_2() {
        let paths = vec![
            vec![str!("B"), str!("C")],
            vec![str!("D"), str!("B")],
            vec![str!("C"), str!("A")],
        ];
        let result = Solution::dest_city(paths);
        assert_eq!(result, "A");
    }

    #[test]
    fn example_3() {
        let paths = vec![
            vec![str!("A"), str!("Z")]
        ];
        let result = Solution::dest_city(paths);
        assert_eq!(result, "Z");
    }

}
