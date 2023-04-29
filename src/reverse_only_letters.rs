struct Solution;

impl Solution {

    pub fn reverse_only_letters(s: String) -> String {
        let mut items: Vec<char> = s.chars().collect();
        let mut start = 0;
        let mut end = s.len() - 1;

        while start < end {
            let start_item = items[start];
            let end_item = items[end];

            if start_item.is_alphabetic() && end_item.is_alphabetic() {
                items.swap(start, end);
                start += 1;
                end -= 1;
            } else if !start_item.is_alphabetic() {
                start += 1;
            } else {
                end -= 1;
            }
        }
        items.iter().collect::<String>()
    }

}
#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn example_1() {
        let s = "ab-cd".to_string();
        let result = Solution::reverse_only_letters(s);
        assert_eq!(result, "dc-ba");
    }

    #[test]
    fn example_2() {
        let s = "a-bC-dEf-ghIj".to_string();
        let result = Solution::reverse_only_letters(s);
        assert_eq!(result, "j-Ih-gfE-dCba");
    }

    #[test]
    fn example_3() {
        let s = "Test1ng-Leet=code-Q!".to_string();
        let result = Solution::reverse_only_letters(s);
        assert_eq!(result, "Qedo1ct-eeLg=ntse-T!");
    }

}
