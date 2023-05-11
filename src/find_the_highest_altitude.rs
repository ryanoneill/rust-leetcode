/// There is a biker going on a road trip. The road trip consists of `n + 1`
/// points at different altitudes. The biker starts his trip on point `0` with
/// altitude equal `0`.
///
/// You are given an integer array `gain` of length `n` where `gain[i]` is the
/// net gain in altitude between points `i` and `i + 1` for all (`0 <= i < n`).
/// Return the highest altitude of a point.
struct Solution;

impl Solution {

    pub fn largest_altitude(gain: Vec<i32>) -> i32 {
        let mut alt = 0;
        let mut max_alt = alt;

        for g in gain {
            alt += g;
            max_alt = max_alt.max(alt);
        }

        max_alt
    }

}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn example_1() {
        let gain = vec![-5,1,5,0,-7];
        let result = Solution::largest_altitude(gain);
        assert_eq!(result, 1);
    }

    #[test]
    fn example_2() {
        let gain = vec![-4,-3,-2,-1,4,3,2];
        let result = Solution::largest_altitude(gain);
        assert_eq!(result, 0);
    }

}
