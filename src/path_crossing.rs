use std::collections::HashSet;

/// Given a string `path`, where `path[i] = 'N'`, `'S'`, `'E'`, or `'W'`, each representing moving
/// one unit north, south, east, or west, respectively. You start at the origin `(0, 0)` on a 2D
/// plane and walk on the path specified by `path`.
///
/// Return `true` if the path crosses itself at any point, that is, if at any time you are on a
/// location you have previously visited. Return `false` otherwise.
struct Solution;

impl Solution {

    pub fn is_path_crossing(path: String) -> bool {
        let n = path.len();
        let mut seen = HashSet::with_capacity(n);
        
        let mut position: (i32, i32) = (0, 0);
        seen.insert(position);

        let mut result = false;

        for dir in path.chars() {
            match dir {
                'N' => { position = (position.0, position.1 + 1); }
                'S' => { position = (position.0, position.1 - 1); }
                'E' => { position = (position.0 + 1, position.1); }
                'W' => { position = (position.0 - 1, position.1); }
                _   => { }
            }
            if seen.contains(&position) {
                result = true;
                break;
            } else {
                seen.insert(position);
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
        let path = str!("NES");
        let result = Solution::is_path_crossing(path);
        assert!(!result);
    }

    #[test]
    fn example_2() {
        let path = str!("NESWW");
        let result = Solution::is_path_crossing(path);
        assert!(result);
    }

}
