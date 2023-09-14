/// Given two binary strings `a` and `b`, return their sum as a binary string.
struct Solution;

impl Solution {

    pub fn add_binary(a: String, b: String) -> String {
        let a_bits: Vec<char> = a.chars().collect();
        let b_bits: Vec<char> = b.chars().collect();

        let mut result = Vec::new();

        let m = a_bits.len();
        let n = b_bits.len();

        let mut carry = false;
        let mut i = 0;
        let mut j = 0;

        let mut a_value;
        let mut b_value;

        loop {
            if i >= m && j >= n {
                break;
            }
            if i >= m {
                a_value = '0';
            } else {
                a_value = a_bits[m-i-1];
            }
            if j >= n {
                b_value = '0';
            } else {
                b_value = b_bits[n-j-1];
            }
            match (a_value, b_value) {
                ('0', '0') => {
                    if carry {
                        result.push('1');
                    } else {
                        result.push('0');
                    }
                    carry = false;
                }
                ('0', '1') => {
                    if carry {
                        result.push('0');
                        carry = true;
                    } else {
                        result.push('1');
                        carry = false;
                    }
                }
                ('1', '0') => {
                    if carry {
                        result.push('0');
                        carry = true;
                    } else {
                        result.push('1');
                        carry = false;
                    }
                }
                ('1', '1') => {
                    if carry {
                        result.push('1');
                    } else {
                        result.push('0');
                    }
                    carry = true;
                }
                (_, _) => {
                    break;
                }
            }
            i += 1;
            j += 1;
        }
        if carry {
            result.push('1');
        }
        result.reverse();
        result.into_iter().collect()
    }

}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn example_1() {
        let a = "11".to_string();
        let b = "1".to_string();
        let result = Solution::add_binary(a, b);
        assert_eq!(result, "100");
    }

    #[test]
    fn example_2() {
        let a = "1010".to_string();
        let b = "1011".to_string();
        let result = Solution::add_binary(a, b);
        assert_eq!(result, "10101");
    }

}
