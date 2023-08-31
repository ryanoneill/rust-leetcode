#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct Complex {
    real: i32,
    imaginary: i32,
}

impl Complex {

    pub fn new(real: i32, imaginary: i32) -> Self {
        Self { real, imaginary }
    }

    pub fn multiply(&self, other: Complex) -> Self {
        let part_a = self.real * other.real;
        let part_b = self.real * other.imaginary;
        let part_c = self.imaginary * other.real;
        let part_d = self.imaginary * other.imaginary;

        let real = part_a - part_d;
        let imaginary = part_b + part_c;

        Self { real, imaginary }
    }

    pub fn parse(s: String) -> Self {
        let n = s.len();
        let plus = s.find('+').unwrap();
        let real = &s[0..plus].parse::<i32>().unwrap();
        let imaginary = &s[plus+1..n-1].parse::<i32>().unwrap();
        Self::new(*real, *imaginary)
    }

    pub fn to_string(&self) -> String {
        let mut result = String::new();
        result.push_str(&self.real.to_string());
        result.push('+');
        result.push_str(&self.imaginary.to_string());
        result.push('i');

        result
    }

}

/// A complex number can be represented as a string on the form "real+imaginaryi" where:
///
/// * `real` is the real part of and is an integer in the range `[-100, 100]`.
///
/// * `imaginary` is the imaginary part and is an integer in the range `[-100, 100]`.
///
/// * `i^2 == -1`.
///
/// Given two complex numbers `num1` and `num2` as strings, return a string of the complex number
/// that represents their multiplications.
struct Solution;

impl Solution {

    pub fn complex_number_multiply(num1: String, num2: String) -> String {
        let c1 = Complex::parse(num1);
        let c2 = Complex::parse(num2);

        let combined = c1.multiply(c2);
        combined.to_string()
    }

}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn example_1() {
        let num1 = "1+1i".to_string();
        let num2 = "1+1i".to_string();
        let result = Solution::complex_number_multiply(num1, num2);
        assert_eq!(result, "0+2i");
    }

    #[test]
    fn example_2() {
        let num1 = "1+-1i".to_string();
        let num2 = "1+-1i".to_string();
        let result = Solution::complex_number_multiply(num1, num2);
        assert_eq!(result, "0+-2i");
    }

}
