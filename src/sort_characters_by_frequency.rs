/// Given a string `s`, sort it in decreasing order based on the frequency of the characters. The
/// frequency of a character is the number of times it appears in the string.
///
/// Return the sorted string. If there are multiple answers, return any of them.
struct Solution;

impl Solution {

    fn to_frequencies(s: String) -> Vec<usize> {
        let mut result = vec![0; 62]; // 10 + 26 + 26 for digits, lowercase, and uppercase
        for letter in s.chars() {
            let index = if letter.is_digit(10) {
                (letter as u32 - '0' as u32) as usize
            } else if letter.is_ascii_uppercase() {
                (letter as u32 - 'A' as u32 + 10) as usize
            } else {
                (letter as u32 - 'a' as u32 + 10 + 26) as usize
            };
            result[index] += 1;
        }
        result
    }

    fn to_buckets(frequencies: Vec<usize>) -> Vec<Vec<char>> {
        let max_frequency = frequencies.iter().max().copied().unwrap_or(0);
        let mut result: Vec<Vec<char>> = vec![Vec::new(); max_frequency + 1];
        for (i, &freq) in frequencies.iter().enumerate() {
            if freq > 0 {
                let letter = if i < 10 {
                    char::from_u32(i as u32 + '0' as u32).unwrap()
                } else if i < 36 {
                    char::from_u32(i as u32 - 10 + 'A' as u32).unwrap()
                } else {
                    char::from_u32(i as u32 - 10 - 26 + 'a' as u32).unwrap()
                };
                result[freq].push(letter);
            }
        }
        result
    }

    pub fn frequency_sort(s: String) -> String {
        let mut result = String::from("");

        let frequencies = Self::to_frequencies(s);
        let buckets = Self::to_buckets(frequencies);

        for (count, bucket) in buckets.iter().enumerate().rev() {
            for letter in bucket {
                result += &letter.to_string().repeat(count);
            }
        }

        result
    }

}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn example_1() {
        let s = "tree".to_string();
        let result = Solution::frequency_sort(s);
        assert_eq!(result, "eert");
    }

    #[test]
    fn example_2() {
        let s = "cccaaa".to_string();
        let result = Solution::frequency_sort(s);
        assert_eq!(result, "aaaccc");
    }

    #[test]
    fn example_3() {
        let s = "Aabb".to_string();
        let result = Solution::frequency_sort(s);
        assert_eq!(result, "bbAa");
    }

    #[test]
    fn digits() {
        let s = "2a554442f544asfasssffffasss".to_string();
        let result = Solution::frequency_sort(s);
        assert_eq!(result, "sssssssffffff44444aaaa55522")
    }

}
