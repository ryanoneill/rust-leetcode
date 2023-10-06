/// Convert a non-negative integer `num` to its English words representation.
struct Solution;

impl Solution {

    // This function does not include zero.
    // Zero is a special case and is only returned when the original value was 0.
    fn digit_to_word(digit: i32) -> String {
        let result = match digit {
            1 => "One",
            2 => "Two",
            3 => "Three",
            4 => "Four",
            5 => "Five",
            6 => "Six",
            7 => "Seven",
            8 => "Eight",
            9 => "Nine",
            _ => "",
        };
        result.to_string()
    }

    /// Special Words:
    /// One, Two, Three, Four, Five, Six, Seven, Eight, Nine, Ten,
    /// Eleven, Twelve, Thirteen, Fourteen, Fifteen, Sixteen, Seventeen, Eighteen, Nineteen, Twenty
    /// Thirty,
    /// Forty,
    /// Fifty
    /// Sixty,
    /// Seventy
    /// Eighty,
    /// Ninety,
    fn part_to_words(value: i32) -> String {
        let mut value = value;
        let mut result = String::from("");

        if value >= 100 {
            let count = value / 100;
            result.push_str(&Self::digit_to_word(count));
            result.push_str(" Hundred");
            value -= count * 100;
        }
        if value >= 90 {
            value -= 90;
            if result != "" {
                result.push_str(" ");
            }
            result.push_str("Ninety");
        } else if value >= 80 {
            value -= 80;
            if result != "" {
                result.push_str(" ");
            }
            result.push_str("Eighty");
        } else if value >= 70 {
            value -= 70;
            if result != "" {
                result.push_str(" ");
            }
            result.push_str("Seventy");
        } else if value >= 60 {
            value -= 60;
            if result != "" {
                result.push_str(" ");
            }
            result.push_str("Sixty");
        } else if value >= 50 {
            value -= 50;
            if result != "" {
                result.push_str(" ");
            }
            result.push_str("Fifty");
        } else if value >= 40 {
            value -= 40;
            if result != "" {
                result.push_str(" ");
            }
            result.push_str("Forty");
        } else if value >= 30 {
            value -= 30;
            if result != "" {
                result.push_str(" ");
            }
            result.push_str("Thirty");
        } else if value >= 20 {
            value -= 20;
            if result != "" {
                result.push_str(" ");
            }
            result.push_str("Twenty");
        } else if value >= 19 {
            value -= 19;
            if result != "" {
                result.push_str(" ");
            }
            result.push_str("Nineteen");
        } else if value >= 18 {
            value -= 18;
            if result != "" {
                result.push_str(" ");
            }
            result.push_str("Eighteen");
        } else if value >= 17 {
            value -= 17;
            if result != "" {
                result.push_str(" ");
            }
            result.push_str("Seventeen");
        } else if value >= 16 {
            value -= 16;
            if result != "" {
                result.push_str(" ");
            }
            result.push_str("Sixteen");
        } else if value >= 15 {
            value -= 15;
            if result != "" {
                result.push_str(" ");
            }
            result.push_str("Fifteen");
        } else if value >= 14 {
            value -= 14;
            if result != "" {
                result.push_str(" ");
            }
            result.push_str("Fourteen");
        } else if value >= 13 {
            value -= 13;
            if result != "" {
                result.push_str(" ");
            }
            result.push_str("Thirteen");
        } else if value >= 12 {
            value -= 12;
            if result != "" {
                result.push_str(" ");
            }
            result.push_str("Twelve");
        } else if value >= 11 {
            value -= 11;
            if result != "" {
                result.push_str(" ");
            }
            result.push_str("Eleven");
        } else if value >= 10 {
            value -= 10;
            if result != "" {
                result.push_str(" ");
            }
            result.push_str("Ten");
        }

        if value > 0 {
            if result != "" {
                result.push_str(" ");
            }
            result.push_str(&Self::digit_to_word(value));
        }

        result
    }

    pub fn number_to_words(value: i32) -> String {
        let mut result = String::from("");
        if value == 0 {
            result = "Zero".to_string();
        } else {
            let mut value = value;
            if value >= 1_000_000_000 {
                let count = value / 1_000_000_000;
                result.push_str(&Self::digit_to_word(count));
                result.push_str(" Billion");
                value -= 1_000_000_000 * count;
            }
            if value >= 1_000_000 {
                let count = value / 1_000_000;
                if result != "" {
                    result.push_str(" ");
                }
                result.push_str(&Self::part_to_words(count));
                result.push_str(" Million");
                value -= 1_000_000 * count;
            }
            if value >= 1_000 {
                let count = value / 1_000;
                if result != "" {
                    result.push_str(" ");
                }
                result.push_str(&Self::part_to_words(count));
                result.push_str(" Thousand");
                value -= 1_000 * count;
            }
            if value >= 1 {
                if result != "" {
                    result.push_str(" ");
                }
                result.push_str(&Self::part_to_words(value));
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
        let num = 123;
        let result = Solution::number_to_words(num);
        assert_eq!(result, "One Hundred Twenty Three");
    }

    #[test]
    fn example_2() {
        let num = 12345;
        let result = Solution::number_to_words(num);
        assert_eq!(result, "Twelve Thousand Three Hundred Forty Five");
    }

    #[test]
    fn example_3() {
        let num = 1234567;
        let result = Solution::number_to_words(num);
        assert_eq!(result, "One Million Two Hundred Thirty Four Thousand Five Hundred Sixty Seven")
    }

    #[test]
    fn example_4() {
        let num = 0;
        let result = Solution::number_to_words(num);
        assert_eq!(result, "Zero");
    }

    #[test]
    fn example_5() {
        let num = 12;
        let result = Solution::number_to_words(num);
        assert_eq!(result, "Twelve");
    }

}

// 1. Write down the problem ✓
// 2. Clarify the problem scope ✓
//    ** Input: i32
//    ** Output: String (multiple words separated by spaces)
//    ** Lower Bound: Zero
//    ** Upper Bound: i32::MAX (2^31-1)
//    ** No 'and' in output
//    ** No hyphens in output
// 3. Write down the test cases ✓
//    ** Input: 123     Output: "One Hundred Twenty Three"
//    ** Input: 12345   Output: "Twelve Thousand Three Hundred Forty Five"
//    ** Input: 1234567 Output: "One Million Two Hundred Thirty Four Thousand Five Hundred Sixty
//       Seven"
//
//    ** Input: 2147483647  Output: "Two Billion One Hundred Forty Seven Million Four Hundred
//    Eighty Three Thousand Six Hundred Forty Seven"
//
// 4. Describe and Write Down the Algorithm ✓
//    ** Function for converting 3 digit number to words (e.g. 942 for Nine Hundred Forty Two)
//       Billion/Million/Thousand
//    ** Input is Zero?
//       ** Return 'Zero' and Exit
//    ** Input larger than a billion? 
//       ** Divide number by a billion, convert the integer value to words
//       ** Add 'Billion' after it
//       ** Subtract that many billion from number
//    ** Input larger than a million?
//       ** Divide number by a million, convert the integer value to words
//       ** Add 'Million' after it
//       ** Subtract that many million from number
//    ** Input larger than a thousand?
//       ** Divide number by a thousand, conver the integer value to words
//       ** Add 'Thousand' after it
//       ** Subtract that many thousands from number
//    ** Input larger than zero?
//       ** Convert the integer value to words
//
//    ** Return
//
// 5. Start Coding ✓
// 6. Test ✓
