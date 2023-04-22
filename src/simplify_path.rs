use crate::stack::Stack;

/// Given a string `path`, which is an absolute path (starting with a slash
/// '/') to a file or directory in a Unix-style file system, convert it to
/// the simplified canonical path.
///
/// In a Unix-style file system, a period '.' refers to the current directory,
/// a double period '..' refers to the directory up a level, and any multiple
/// consecutive slashes (i.e. '//') are treated as a single slash '/'. For this
/// problem, any other format of periods such as '...' are treated as
/// file/directory names.
///
/// The canonical path should have the following format:
/// * The path starts with a single slash '/'.
/// * Any two directories are separated by a single slash '/'.
/// * The path does not end with a trailing '/'.
/// * The path only contains the directories on the path from the root
///   directory to the target file or directory (i.e., no period '.' or double
///   period '..')
///
/// Return the simplified canonical path.
struct Solution;

impl Solution {

    pub fn simplify_path(path: String) -> String {
        let mut stack = Stack::new();
        for part in path.split('/') {
            match part {
                "" | "." => { } // ignore empty or same directory
                ".." => { stack.pop(); }
                _ => { stack.push(part); }
            }
        }
        let mut result = String::from("/");
        result.push_str(&stack.join("/"));
        result
    }

}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn example_1() {
        let path = "/home/".to_string();
        let result = Solution::simplify_path(path);
        assert_eq!(result, "/home");
    }

    #[test]
    fn example_2() {
        let path = "/../".to_string();
        let result = Solution::simplify_path(path);
        assert_eq!(result, "/");
    }

    #[test]
    fn example_3() {
        let path = "/home//foo/".to_string();
        let result = Solution::simplify_path(path);
        assert_eq!(result, "/home/foo");
    }

    #[test]
    fn bells_and_whistles() {
        let path = "/home/../usr//local/./bin/".to_string();
        let result = Solution::simplify_path(path);
        assert_eq!(result, "/usr/local/bin");
    }

}
