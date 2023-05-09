use std::collections::HashMap;
use std::collections::HashSet;
use std::collections::VecDeque;

/// You are given an integer `n`, which indicates that there are `n` courses
/// labeled from `1` to `n`. You are also given an array `relations` where
/// `relations[i] = [prevCoursei, nextCoursei]`, representing a prerequisite
/// relationship between course `prevCoursei` and course `nextCoursei`: course
/// `prevCoursei` has to be taken before course `nextCoursei`.
///
/// In one semester, you can take any number of courses as long as you have
/// taken all the prerequisites in the previous semester for the courses you
/// are taking.
///
/// Return the minimum number of semesters needed to take all courses. If there
/// is no way to take all the courses, return `-1`.
struct Solution;

impl Solution {

    pub fn minimum_semesters(n: i32, relations: Vec<Vec<i32>>) -> i32 {
        let n = n as usize;

        let mut result = 0;
        let mut in_degrees = vec![0; n+1];
        let mut prereq_for = HashMap::new();

        for rel in &relations {
            let prereq = rel[0] as usize;
            let for_class = rel[1] as usize;

            prereq_for
                .entry(prereq)
                .or_insert(HashSet::new())
                .insert(for_class);
            in_degrees[for_class] += 1;
        }

        let mut seen = HashSet::new();
        let mut queue = VecDeque::new();

        for i in 1..=n {
            if in_degrees[i] == 0 {
                queue.push_back(i);
                seen.insert(i);
            }
        }

        while !queue.is_empty() {
            let q = queue.len();
            result += 1;
            for _ in 0..q {
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
        }

        if seen.len() == n {
            result as i32
        } else { -1 }
    }

}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn example_1() {
        let n = 3;
        let relations = vec![vec![1,3], vec![2,3]];
        let result = Solution::minimum_semesters(n, relations);
        assert_eq!(result, 2);
    }

    #[test]
    fn example_2() {
        let n = 3;
        let relations = vec![vec![1,2], vec![2,3], vec![3,1]];
        let result = Solution::minimum_semesters(n, relations);
        assert_eq!(result, -1);
    }

}
