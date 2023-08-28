#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct Item {
    index: usize,
    height: i32,
}

impl Item {

    pub fn new(index: usize, height: i32) -> Self {
        Self { index, height }
    }

}

/// Given an array of integers `heights` representing the histogram's bar heigh where the width of
/// each bar is `1`, return the area of the largest rectangle in the histogram.
struct Solution;

impl Solution {

    fn top_height(stack: &Vec<Item>) -> i32 {
        let n = stack.len();
        if n == 0 {
            0
        } else {
            stack[n-1].height
        }
    }

    /// 1 <= heights.length <= 10^5
    /// 0 <= heights[i] <= 10^4
    pub fn largest_rectangle_area(heights: Vec<i32>) -> i32 {
        let n = heights.len();

        let mut result = 0;
        let mut stack: Vec<Item> = Vec::new();

        for i in 0..n {
            let h = heights[i];
            let mut start = i;
            while Self::top_height(&stack) > h {
                let item = stack.pop().unwrap();
                result = result.max(item.height * (i - item.index) as i32);
                start = item.index;
            }
            stack.push(Item::new(start, h));
        }

        for item in stack {
            result = result.max(item.height * (n - item.index) as i32);
        }
        result
    }

}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn example_1() {
        let heights = vec![2,1,5,6,2,3];
        let result = Solution::largest_rectangle_area(heights);
        assert_eq!(result, 10);
    }

    #[test]
    fn example_2() {
        let heights = vec![2,4];
        let result = Solution::largest_rectangle_area(heights);
        assert_eq!(result, 4);
    }

}
