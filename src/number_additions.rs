// TODO: Make this more robust
pub trait NumberAdditions {

    fn to_vec(&self) -> Vec<u32>;
    fn from_vec(digits: Vec<u32>) -> Self;

}

impl NumberAdditions for u32 {

    fn to_vec(&self) -> Vec<u32> {

        let mut n = *self;
        let mut result = Vec::new();
        loop {
            let digit = n % 10;
            result.push(digit);
            if n == digit {
                break;
            } else {
                n /= 10;
            }
        }

        result.into_iter().rev().collect()
    }

    // This does not currently handle large numbers
    fn from_vec(digits: Vec<u32>) -> Self {
        let mut result = 0;

        let n = digits.len();
        let mut power = 0;
        for i in 0..n {
            let digit = digits[n-1-i];
            let piece = digit * (10_u32.pow(power));
            result += piece;
            power += 1;
        }

        result
    }

}

#[cfg(test)]
mod tests {
    use super::NumberAdditions;

    #[test]
    fn to_single_digit_vec() {
        let n = 5;
        let result = n.to_vec();
        assert_eq!(result, vec![5]);
    }

    #[test]
    fn to_multi_digit_vec() {
        let n = 13579246;
        let result = n.to_vec();
        assert_eq!(result, vec![1,3,5,7,9,2,4,6]);
    }

    #[test]
    fn to_zero_vec() {
        let n = 0;
        let result = n.to_vec();
        assert_eq!(result, vec![0]);
    }

    #[test]
    fn from_single_digit_vec() {
        let digits = vec![8];
        let result: u32 = NumberAdditions::from_vec(digits);
        assert_eq!(result, 8);
    }

    #[test]
    fn from_multi_digit_vec() {
        let digits = vec![5,4,3,2,1];
        let result: u32 = NumberAdditions::from_vec(digits);
        assert_eq!(result, 54321);
    }

    #[test]
    fn from_zero_vec() {
        let digits = vec![0];
        let result: u32 = NumberAdditions::from_vec(digits);
        assert_eq!(result, 0);
    }

}
