/// Given two strings, `s` and `t`, return `true` if they are equal when both are typed into empty
/// text editors. `'#'` means a backspace character.
///
/// Note that after backspacing an empty text, the text will continue empty.
struct Solution;

impl Solution {

    fn evaluate(s: String) -> String {
        let mut letters = Vec::new();
        for c in s.chars() {
            match c {
                '#' => {
                    if letters.len() > 0 {
                        letters.pop();
                    }
                }
                _ => {
                    letters.push(c);
                }
            }
        }
        letters.into_iter().collect()
    }

    pub fn backspace_compare(s: String, t: String) -> bool {
        Self::evaluate(s) == Self::evaluate(t)
    }

}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn example_1() {
        let s = str!("ab#c");
        let t = str!("ad#c");
        assert!(Solution::backspace_compare(s, t));
    }

    #[test]
    fn example_2() {
        let s = str!("ab##");
        let t = str!("c#d#");
        assert!(Solution::backspace_compare(s, t));
    }

    #[test]
    fn example_3() {
        let s = str!("a#c");
        let t = str!("b");
        assert!(!Solution::backspace_compare(s, t));
    }

}
