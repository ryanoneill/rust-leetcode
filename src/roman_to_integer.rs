/// Roman numerals are represented by seven different symbols: `I`, `V`, `X`,
/// `L`, `C`, `D`, `M`.
///
/// Symbol         Value
/// ------         -----
/// I              1
/// V              5
/// X              10
/// L              50
/// C              100
/// D              500
/// M              1000
///
/// For example, `2` is written as `II` in Roman numeral, just two ones added
/// together. `12` is written as `XII`, which is simply `X + II`. The number
/// `27` is written as `XXVII`, which is `XX + V + II`.
///
/// Roman numerals are usually written largest to smallest from left to right.
/// However, the numeral for four is not `IIII`. Instead the number four is
/// written as `IV`. Because the one is before the five we subtract it making
/// four. The same principle applies to the number nine, which is written as
/// `IX`. There are six instances where subtraction is used: 
///
/// * `I` can be placed before `V` (5) and `X` (10) to make 4 and 9.
///
/// * `X` can be placed before `L` (50) and `C` (100) to make 40 and 90.
///
/// * `C` can be placed before `D` (500) and `M` (1000) to make 400 and 900.
///
/// Given a roman numeral, convert it to an integer.
struct Solution;

impl Solution {

    pub fn roman_to_int(s: String) -> i32 {
        let mut result = 0;
        let numerals: Vec<char> = s.chars().collect();

        let n = numerals.len();

        let mut i = 0;

        while i < n {
            let numeral = numerals[i];
            match numeral {
                'I' => { 
                    if i == n-1 {
                        result += 1;
                    } else {
                        match numerals[i+1] {
                            'V' => { result += 4; i += 1; }
                            'X' => { result += 9; i += 1; }
                            _   => { result += 1; }
                        }
                    }
                }
                'V' => { result += 5; }
                'X' => { 
                    if i == n-1 {
                        result += 10;
                    } else {
                        match numerals[i+1] {
                            'L' => { result += 40; i += 1; }
                            'C' => { result += 90; i += 1; }
                            _   => { result += 10; }
                        }
                    }
                }
                'L' => { result += 50; }
                'C' => { 
                    if i == n-1 {
                        result += 100;
                    } else {
                        match numerals[i+1] {
                            'D' => { result += 400; i += 1; }
                            'M' => { result += 900; i += 1; }
                            _   => { result += 100; }
                        }
                    }
                }
                'D' => { result += 500; }
                'M' => { result += 1000; }
                _ => { }
            }
            i += 1;
        }

        result
    }

}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn example_1() {
        let s = "III".to_string();
        let result = Solution::roman_to_int(s);
        assert_eq!(result, 3);
    }

    #[test]
    fn example_2() {
        let s = "LVIII".to_string();
        let result = Solution::roman_to_int(s);
        assert_eq!(result, 58);
    }

    #[test]
    fn example_3() {
        let s = "MCMXCIV".to_string();
        let result = Solution::roman_to_int(s);
        assert_eq!(result, 1994);
    }

}
