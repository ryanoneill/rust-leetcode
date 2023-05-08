use std::collections::HashMap;
use std::collections::HashSet;
use std::collections::VecDeque;

/// There are a total of `numCourses` courses you have to take, labeled from
/// `0` to `numCourses - 1`. You are given an array `prerequisites` where
/// `prerequisites[i] = [ai, bi]` indicates that you must take course `bi`
/// first if you want to take course `ai`.
///
/// * For example, the pair `[0, 1]`, indicates that to take a course `0` you
/// have to first take course `1`.
///
/// Return `true` if you can finish all courses. Otherwise, return `false`.
struct Solution;

impl Solution {

    pub fn can_finish(num_courses: i32, prerequisites: Vec<Vec<i32>>) -> bool {
        let n = num_courses as usize;

        let mut in_degrees = vec![0; n];
        let mut out_degrees = vec![0; n];
        let mut prereq_for = HashMap::new();

        for p in prerequisites {
            let prereq = p[1] as usize;
            let for_class = p[0] as usize;
            prereq_for
                .entry(prereq)
                .or_insert(HashSet::new())
                .insert(for_class);
            out_degrees[prereq] += 1;
            in_degrees[for_class] += 1;
        }

        let mut seen = HashSet::new();
        let mut queue = VecDeque::new();

        for i in 0..n {
            if in_degrees[i] == 0 {
                queue.push_back(i);
                seen.insert(i);
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
                            }
                        }
                    }
                }
                None => { }
            }
        }

        seen.len() == n
    }

}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn example_1() {
        let num_courses = 2;
        let prerequisites = vec![vec![1,0]];
        let result = Solution::can_finish(num_courses, prerequisites);
        assert!(result);
    }

    #[test]
    fn example_2() {
        let num_courses = 2;
        let prerequisites = vec![vec![1,0], vec![0,1]];
        let result = Solution::can_finish(num_courses, prerequisites);
        assert!(!result);
    }

    #[test]
    fn sedgewick_example() {
        let num_courses = 13;
        let prerequisites = vec![
            vec![0,2], vec![1,0], vec![3,2], vec![4,5], vec![4,6], vec![5,0], vec![5,3],
            vec![6,7], vec![7,8], vec![9,6], vec![10,9], vec![11,9], vec![12,9], vec![12,11]
        ];

        let result = Solution::can_finish(num_courses, prerequisites);
        assert!(result);
    }

}
