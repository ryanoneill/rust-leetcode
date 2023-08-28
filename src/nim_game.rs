/// You are playing the following Nim Game with your friend:
///
/// * Initially, there is a heap of stones on the table.
///
/// * You and your friend will alternate taking turns, and you go first.
///
/// * On each turn, the person whose turn it is will remove 1 to 3 stones from the heap.
///
/// * The one who removes the last stone is the winner.
///
/// Given `n`, the number of stones in the heap, return `true` if you can win the game assuming
/// both you and your friend play optimally, otherwise return `false`.
struct Solution;

impl Solution {

    pub fn can_win_nim(n: i32) -> bool {
        n % 4 != 0
    }

}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn example_1() {
        let n = 4;
        let result = Solution::can_win_nim(n);
        assert!(!result);
    }

    #[test]
    fn example_2() {
        let n = 1;
        let result = Solution::can_win_nim(n);
        assert!(result);
    }

    #[test]
    fn example_3() {
        let n = 2;
        let result = Solution::can_win_nim(n);
        assert!(result);
    }

}
