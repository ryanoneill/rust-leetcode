use std::collections::HashMap;
use std::collections::HashSet;
use std::collections::VecDeque;

/// There are a total of `numCourses` courses you have to take, labeled from
/// `0` to `numCourses - 1`. You are given an array `prerequisites` where
/// `prerequisites[i] = [ai, bi]` indicates that you must take course `bi`
/// first if you want to take course `ai`.
///
/// * For example, the pair `[0,1]`, indicates that to take course `0` you have
///   to first take course `1`.
///
/// Return the ordering of courses you should take to finish all courses. If
/// there are many valid answers, return any of them. If it is impossible to
/// finish all courses, return an empty array.
struct Solution;

impl Solution {

    pub fn find_order(num_courses: i32, prerequisites: Vec<Vec<i32>>) -> Vec<i32> {
        let n = num_courses as usize;

        let mut in_degrees = vec![0; n];
        let mut prereq_for = HashMap::new();

        for p in prerequisites {
            let prereq = p[1] as usize;
            let for_class = p[0] as usize;
            prereq_for
                .entry(prereq)
                .or_insert(HashSet::new())
                .insert(for_class);
            in_degrees[for_class] += 1;
        }

        let mut seen = HashSet::new();
        let mut queue = VecDeque::new();
        let mut result = Vec::new();

        for i in 0..n {
            if in_degrees[i] == 0 {
                queue.push_back(i);
                seen.insert(i);
                result.push(i as i32);
            }
        }

        while !queue.is_empty() {
            let current = queue.pop_front().unwrap();
            match prereq_for.get(&current) {
                Some(classes) => {
                    for &class in classes {
                        if !seen.contains(&class) {
                            in_degrees[class] -= 1;
                            if in_degrees[class] == 0 {
                                queue.push_back(class);
                                seen.insert(class);
                                result.push(class as i32);
                            }
                        }
                    }
                }
                None => { }
            }
        }

        if seen.len() != n {
            result = vec![];
        }
        result
    }

}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn example_1() {
        let num_courses = 2;
        let prerequisites = vec![vec![1,0]];
        let result = Solution::find_order(num_courses, prerequisites);
        assert_eq!(result, vec![0,1]);
    }

    #[test]
    fn example_2() {
        let num_courses = 4;
        let prerequisites = vec![vec![1,0], vec![2,0], vec![3,1], vec![3,2]];
        let result = Solution::find_order(num_courses, prerequisites);
        assert!(result == vec![0,1,2,3] || result == vec![0,2,1,3]);
    }

    #[test]
    fn example_3() {
        let num_courses = 1;
        let prerequisites = vec![];
        let result = Solution::find_order(num_courses, prerequisites);
        assert_eq!(result, vec![0]);
    }

    #[test]
    fn cycle() {
        let num_courses = 5;
        let prerequisites = vec![vec![0,1], vec![1,2], vec![2,3], vec![3,4], vec![4,2]];
        let result = Solution::find_order(num_courses, prerequisites);
        assert_eq!(result, vec![]);
    }

}
