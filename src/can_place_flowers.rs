/// You have a long flowerbed in which some of the plots are planted, and some
/// are not. However, flowers cannot be planted in adjacent plots.
///
/// Given an integer array `flowerbed` containing `0`'s and `1`'s where `0`
/// means empty and `1` means not empty, and an integer `n`, return `true` if
/// `n` new flowers can be planted in the `flowerbed` without violating the
/// no-adjacent-flowers rule and `false` otherwise.
struct Solution;

impl Solution {

    pub fn can_place_flowers(flowerbed: Vec<i32>, n: i32) -> bool {
        let mut flowers = n;

        let mut flowerbed = flowerbed;
        let n = flowerbed.len();

        if flowers > 0 {
            for i in 0..n {
                let empty = flowerbed[i] == 0;
                if empty {
                    let can_plant = if n == 1 {
                        true
                    } else if i == 0 {
                        flowerbed[i+1] == 0
                    } else if i == n-1 {
                        flowerbed[n-2] == 0
                    } else {
                        (flowerbed[i-1] == 0) && (flowerbed[i+1] == 0)
                    };
                    if can_plant {
                        flowerbed[i] = 1;
                        flowers -= 1;
                        if flowers == 0 {
                            break;
                        }
                    }
                }
            }
        }

        flowers == 0
    }

}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn example_1() {
        let flowerbed = vec![1,0,0,0,1];
        let n = 1;
        let result = Solution::can_place_flowers(flowerbed, n);
        assert!(result);
    }

    #[test]
    fn example_2() {
        let flowerbed = vec![1,0,0,0,1];
        let n = 2;
        let result = Solution::can_place_flowers(flowerbed, n);
        assert!(!result);
    }

    #[test]
    fn single_empty() {
        let flowerbed = vec![0];
        let n = 1;
        let result = Solution::can_place_flowers(flowerbed, n);
        assert!(result);
    }

}
