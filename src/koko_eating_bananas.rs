/// Koko loves to eat bananas. There are `n` piles of bananas, the `ith` pile
/// has `piles[i]` bananas. The guards have gone and will come back in `h`
/// hours.
///
/// Koko can decide her bananas-per-hour eating speed of `k`. Each hour, she
/// chooses some pile of bananas and eats `k` bananas from that pile. If the
/// pile has less than `k` bananas, she eats all of them instead and will not
/// eat any more bananas during this hour.
///
/// Koko likes to eat slowly but still wants to finish eating all the bananas
/// before the guards return.
///
/// Return the minimum integer `k` such that she can eat all the bananas within
/// `h` hours.
struct Solution;

impl Solution {

    fn max_pile(piles: &Vec<i32>) -> i32 {
        let mut result = 0;
        for i in 0..piles.len() {
            let pile = piles[i];
            result = result.max(pile);
        }
        result
    }


    fn can_finish(piles: &Vec<i32>, hours: i32, speed: i32) -> bool {
        let mut result = 0;
        for pile in piles {
            result += (*pile as f64 / speed as f64).ceil() as i64;
        }
        result <= hours as i64
    }

    pub fn min_eating_speed(piles: Vec<i32>, h: i32) -> i32 {
        let mut start = 1;
        let mut end = Self::max_pile(&piles);

        while start <= end {
            let mid = start + (end - start) / 2;

            if Self::can_finish(&piles, h, mid) {
                end = mid - 1;
            } else {
                start = mid + 1;
            }

        }

        start
    }

}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn example_1() {
        let piles = vec![3,6,7,11];
        let h = 8;
        let result = Solution::min_eating_speed(piles, h);
        assert_eq!(result, 4);
    }

    #[test]
    fn example_2() {
        let piles = vec![30,11,23,4,20];
        let h = 5;
        let result = Solution::min_eating_speed(piles, h);
        assert_eq!(result, 30);
    }

    #[test]
    fn example_3() {
        let piles = vec![805306368,805306368,805306368];
        let h = 1000000000;
        let result = Solution::min_eating_speed(piles, h);
        assert_eq!(result, 3);
    }

}
