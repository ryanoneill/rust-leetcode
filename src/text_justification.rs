/// Given an array of strings `words` and a width `maxWidth`, format the text such that each line
/// has exactly `maxWidth` characters and is fully (left and right) justified.
///
/// You should pack your words in a greedy approach; that is, pack as many words as you can in each
/// line. Pad extra spaces `' '` when necessary so that each line has exactly `maxWidth`
/// characters.
///
/// Extra spaces between words should be distributed as evenly as possible. If the number of spaces
/// on a line does not divide evenly between words, the empty slots on the left will be assigned
/// more spaces than the slots on the right.
///
/// For the last line of text, it should be left-justified, and no extra space is inserted between
/// words.
struct Solution;

impl Solution {

    pub fn full_justify(words: Vec<String>, max_width: i32) -> Vec<String> {
        let max_width = max_width as usize;
        let n = words.len();

        let mut line_len = 0;
        let mut start = 0;

        let mut results = vec![];

        for i in 0..n {
            let word = &words[i];
            let word_len = word.len();
            let attempt = if line_len == 0 { 
                word_len 
            } else {
                line_len + word_len + 1 
            };
            if attempt == max_width {
                // Matches Perfectly
                // One Space Between Words
                let mut line = words[start].clone();
                for j in start+1..=i {
                    line.push(' ');
                    line.push_str(&words[j]);
                }
                results.push(line);
                start = i + 1;
                line_len = 0;
            } else if attempt < max_width {
                // Keep Going
                line_len = attempt;
            } else {
                // Overshot It
                let mut chars_len = 0;
                let mut count = 0;
                for j in start..i {
                    chars_len += words[j].len();
                    count += 1;
                }
                count -= 1;
                let mut diff = max_width - chars_len;
                let mut line = words[start].clone();
                for j in start+1..i {
                    // Divide the spaces evenly between words and
                    // push the spaces and the word.
                    let spaces = (diff as f64 / count as f64).ceil() as usize;
                    count -= 1;
                    diff -= spaces;
                    for _ in 0..spaces {
                        line.push(' ');
                    }
                    line.push_str(&words[j]);
                }
                // If there's only one word in the line, there may
                // need to be spaces pushed at the end.
                for _ in 0..diff {
                    line.push(' ');
                }
                results.push(line);
                start = i;
                line_len = word_len;
            }
        }

        if start < n {
            let mut line = words[start].clone();
            line_len = words[start].len();
            for j in start+1..n {
                line.push(' ');
                line.push_str(&words[j]);
                line_len += 1 + &words[j].len();
            }
            if line_len < max_width {
                for _ in line_len..max_width {
                    line.push(' ');
                }
            }
            results.push(line);
        }

        results
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn example_1() {
        let words = vec![
            str!("This"),
            str!("is"),
            str!("an"),
            str!("example"),
            str!("of"),
            str!("text"),
            str!("justification.")
        ];
        let max_width = 16;
        let results = Solution::full_justify(words, max_width);
        assert_eq!(results, vec![
           "This    is    an",
           "example  of text",
           "justification.  "
        ]);
    }

    #[test]
    fn example_2() {
        let words = vec![
            str!("What"),
            str!("must"),
            str!("be"),
            str!("acknowledgment"),
            str!("shall"),
            str!("be"),
        ];
        let max_width = 16;
        let results = Solution::full_justify(words, max_width);
        assert_eq!(results, vec![
            "What   must   be",
            "acknowledgment  ",
            "shall be        "
        ]);
    }

}
