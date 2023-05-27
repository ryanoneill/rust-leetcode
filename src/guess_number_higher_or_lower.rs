/// We are playing the Guess Game. The game is as follows:
///
/// I pick a number from `1` to `n`. You have to guess which number I picked.
///
/// Every time you guess wrong, I will tell you whether the number I picked
/// is higher or lower than your guess.
///
/// You call a pre-defined API `int guess(int num)`, which returns three
/// possible results:
///
/// * `-1`: Your guess is higher than the number I picked (i.e. `num > pick`).
///
/// * `1`: Your guess is lower than the number I picked (i.e. `num < pick`).
///
/// * `0`: Your guess is equal to the number I picked (i.e. `num == pick`).
///
/// Return the number that I picked.
struct Solution;

static mut PICK: i32 = 0;

impl Solution {

    // There is no reason for this to be labeled unsafe.
    // The signature is defined by LeetCode.
    unsafe fn guess(num: i32) -> i32 {
        if num == PICK {
            0
        } else if num < PICK {
            1
        } else {
            -1
        }
    }

    // This should be guess_number and no reason for it to be labeled unsafe.
    // The signature is defined by LeetCode.
    #[allow(non_snake_case)]
    unsafe fn guessNumber(n: i32) -> i32 {
        let result;
        let mut low = 1;
        let mut high = n;
        loop {
            let mid = low + (high - low) / 2;
            let attempt = Self::guess(mid);
            if attempt == 0 {
                result = mid;
                break;
            } else if attempt == 1 {
                low = mid + 1;
            } else {
                high = mid - 1;
            }
        }

        result
    }

}

#[cfg(test)]
mod tests {
    use super::PICK;
    use super::Solution;

    fn run(n: i32, pick: i32) -> i32 {
        unsafe {
            PICK = pick;
            Solution::guessNumber(n)
        }
    }

    #[test]
    fn example_1() {
        let n = 10;
        let pick = 6;
        let result = run(n, pick);
        assert_eq!(result, pick);
    }

    #[test]
    fn example_2() {
        let n = 1;
        let pick = 1;
        let result = run(n, pick);
        assert_eq!(result, 1);
    }

    #[test]
    fn example_3() {
        let n = 2;
        let pick = 1;
        let result = run(n, pick);
        assert_eq!(result, 1);
    }

}
