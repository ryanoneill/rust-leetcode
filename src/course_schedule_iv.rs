use std::collections::HashMap;
use std::collections::HashSet;
use std::collections::VecDeque;

/// There are a total of `numCourses` courses you have to take, labeled from `0` to `numCourses -
/// 1`. You are given an array `prerequisites` where `prerequisites[i] = [ai, bi]` indicates that
/// you must take course `ai` first if you want to take course `bi`.
///
/// * For example, the pair `[0, 1]` indicates that you have to take course `0` before you can take
///   course `1`.
///
/// Prerequisites can also be indirect. If course `a` is a prerequisite of course `b`, and course
/// `b` is a prerequisite of course `c`, then course `a` is a prerequisite of course `c`.
///
/// You are also given an array `queries` where `queries[j] = [uj, vj]`. For the `jth` query, you
/// should answer whether course `uj` is a prerequisite of course `vj` or not.
///
/// Return a boolean array `answer`, where `answer[j]` is the answer to the `jth` query.
struct Solution;

impl Solution {

    fn is_prereq(prereq_map: &HashMap<i32, HashSet<i32>>, query: &Vec<i32>) -> bool {
        let mut result = false;

        let mut seen = HashSet::new();
        let candidate = query[0];
        let course = query[1];

        let mut queue = VecDeque::new();
        queue.push_back(candidate);
        seen.insert(candidate);
        while !queue.is_empty() {
            let item = queue.pop_front().unwrap();
            if item == course {
                result = true;
                break;
            } else if prereq_map.contains_key(&item) {
                for next in &prereq_map[&item] {
                    if !seen.contains(next) {
                        seen.insert(*next);
                        queue.push_back(*next);
                    }
                }
            }
        }

        result
    }

    fn to_prereq_map(prerequisites: Vec<Vec<i32>>) -> HashMap<i32, HashSet<i32>> {
        let mut results = HashMap::new();

        for item in prerequisites {
            let prereq = item[0];
            let course = item[1];
            results
                .entry(prereq)
                .or_insert(HashSet::new())
                .insert(course);
        }

        results
    }

    pub fn check_if_prerequisite(_num_courses: i32, prerequisites: Vec<Vec<i32>>, queries: Vec<Vec<i32>>) -> Vec<bool> {
        let prereq_map = Self::to_prereq_map(prerequisites);
        let n = queries.len();
        let mut results = Vec::with_capacity(n);

        for query in queries {
            let result = Self::is_prereq(&prereq_map, &query);
            results.push(result);
        }

        results
    }

}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn example_1() {
        let num_courses = 2;
        let prerequisites = vec![vec![1,0]];
        let queries = vec![vec![0,1], vec![1,0]];
        let result = Solution::check_if_prerequisite(num_courses, prerequisites, queries);
        assert_eq!(result, vec![false, true]);
    }

    #[test]
    fn example_2() {
        let num_courses = 2;
        let prerequisites = vec![];
        let queries = vec![vec![1,0], vec![0,1]];
        let result = Solution::check_if_prerequisite(num_courses, prerequisites, queries);
        assert_eq!(result, vec![false, false]);
    }

    #[test]
    fn example_3() {
        let num_courses = 3;
        let prerequisites = vec![vec![1,2], vec![1,0], vec![2,0]];
        let queries = vec![vec![1,0], vec![1,2]];
        let result = Solution::check_if_prerequisite(num_courses, prerequisites, queries);
        assert_eq!(result, vec![true, true]);
    }

}
