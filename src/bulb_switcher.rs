struct Solution {

}

impl Solution {

    pub fn bulb_switch(n: i32) -> i32 {
        (n as f64).sqrt().floor() as i32
    }

}

#[cfg(test)]
mod tests {
    use super::Solution;


    #[test]
    fn example_1() {
        let n = 3;
        let result = Solution::bulb_switch(n);
        assert_eq!(result, 1);
    }

    #[test]
    fn example_2() {
        let n = 0;
        let result = Solution::bulb_switch(n);
        assert_eq!(result, 0);
    }

    #[test]
    fn example_3() {
        let n = 1;
        let result = Solution::bulb_switch(n);
        assert_eq!(result, 1);
    }

    #[test]
    fn real_world_1() {
        let n = 10;
        let result = Solution::bulb_switch(n);
        assert_eq!(result, 3);
    }

    #[test]
    fn real_world_2() {
        let n = 24;
        let result = Solution::bulb_switch(n);
        assert_eq!(result, 4);
    }

    #[test]
    fn real_world_3() {
        let n = 25;
        let result = Solution::bulb_switch(n);
        assert_eq!(result, 5);
    }

    #[test]
    fn real_world_4() {
        let n = 99999999;
        let result = Solution::bulb_switch(n);
        assert_eq!(result, 9999);
    }

    // 0 -> 0
    // 1 -> 1
    // 2 -> 1
    // 3 -> 1
    // 4 -> 2
    // 5 -> 2
    // 6 -> 2
    // 7 -> 2
    // 8 -> 2
    // 9 -> 3
    // 10 -> 3
    // 11 -> 3
    // 12 -> 3
    // 13 -> 3
    // 14 -> 3
    // 15 -> 3
    // 16 -> 4
    // 17 -> 4
    // 18 -> 4
    // 19 -> 4
    // 20 -> 4
    // 21 -> 4
    // 22 -> 4
    // 23 -> 4
    // 24 -> 4
    // 25 -> 5



}

// X Y Y X Y Y

// 10
// 1 2 3 4 5 6 7 8 9 10 (1) (10)
// 1 X 3 X 5 X 7 X 9 X  (2) (5)
// 1 X X X 5 6 7 X X X  (3) (4)
// 1 X X 4 5 6 7 8 X X  (4) (6)
// 1 X X 4 X 6 7 8 X 0  (5) (6)
// 1 X X 4 X X 7 8 X 0  (6) (5)
// 1 X X 4 X X X 8 X 0  (7) (4)
// 1 X X 4 X X X X X 0  (8) (3)
// 1 X X 4 X X X X 9 0  (9) (4)
// 1 X X 4 X X X X 9 X  (10) (3)
