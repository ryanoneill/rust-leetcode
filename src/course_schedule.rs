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

    // TODO: Implement
    pub fn can_finish(_num_courses: i32, _prerequisites: Vec<Vec<i32>>) -> bool {
        false
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

}
