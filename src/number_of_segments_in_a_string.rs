/// Given a string `s`, return the number of segments in the string.
///
/// A segment is defined to be a contiguous sequence of non-space characters.
struct Solution;

impl Solution {

    pub fn count_segments(s: String) -> i32 {
        let mut result = 0;
        let mut last_was_space = true;

        for c in s.chars() {
            match c {
                ' ' => {
                    if !last_was_space {
                        result += 1;
                        last_was_space = true;
                    }
                }
                _ => {
                    last_was_space = false;
                }
            }
        }
        if !last_was_space {
            result += 1;
        }
        
        result
    }

}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn example_1() {
        let s = str!("Hello, my name is John");
        let result = Solution::count_segments(s);
        assert_eq!(result, 5);
    }

    #[test]
    fn example_2() {
        let s = str!("Hello");
        let result = Solution::count_segments(s);
        assert_eq!(result, 1);
    }

    #[test]
    fn example_3() {
        let s = str!("");
        let result = Solution::count_segments(s);
        assert_eq!(result, 0);
    }

    #[test]
    fn example_4() {
        let s = str!("                ");
        let result = Solution::count_segments(s);
        assert_eq!(result, 0);
    }

}
