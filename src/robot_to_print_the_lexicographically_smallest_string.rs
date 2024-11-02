/// You are given a string `s` and a robot that currently holds an empty string `t`. Apply one of
/// the following operations until `s` and `t` are both empty:
///
/// * Remove the first character of a string `s` and give it to the robot. The robot will append
///   this character to the string `t`.
///
/// * Remove the last character of a string `t` and give it to the robot. The robot will write this
///   character on paper.
///
/// Return the lexicographically smallest string that can be written on the paper.
struct Solution;

impl Solution {

    pub fn robot_with_string(s: String) -> String {
        let n = s.len();
        let letters: Vec<char> = s.chars().collect();

        let mut min_suffix: Vec<char> = vec![' '; n];
        let mut minimum = 'z';

        let mut result = Vec::with_capacity(n);

        for i in (0..n).rev() {
            let letter = letters[i];
            minimum = minimum.min(letter);
            min_suffix[i] = minimum;
        }

        let mut buffer = Vec::new();

        for i in 0..n {
            let letter = letters[i];
            if buffer.is_empty() {
                buffer.push(letter);
            } else {
                loop {
                    let last = buffer[buffer.len() - 1];
                    let suffix = min_suffix[i];

                    if last <= suffix {
                        result.push(last);
                        buffer.pop();
                        if buffer.is_empty() {
                            break;
                        }
                    } else {
                        break;
                    }
                }
                buffer.push(letter);
            }
        }
        while !buffer.is_empty() {
            let last = buffer.pop().unwrap();
            result.push(last);
        }

        result.iter().collect()
    }

}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn example_1() {
        let s = str!("zza");
        let result = Solution::robot_with_string(s);
        assert_eq!(result, "azz");
    }

    #[test]
    fn example_2() {
        let s = str!("bac");
        let result = Solution::robot_with_string(s);
        assert_eq!(result, "abc");
    }

    #[test]
    fn example_3() {
        let s = str!("bdda");
        let result = Solution::robot_with_string(s);
        assert_eq!(result, "addb");
    }

    #[test]
    fn example_4() {
        let s = str!("mmuqezwmomeplrtskz");
        let result = Solution::robot_with_string(s);
        assert_eq!(result, "eekstrlpmomwzqummz");
    }

}
