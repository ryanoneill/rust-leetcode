use std::collections::BinaryHeap;

/// Assume you are an awesome parent and want to give your children some cookies. But, you should
/// give each child at most one cookie.
///
/// Each child `i` has a greed factor `g[i]`, which is the minimum size of a cookie that the child
/// will be content with; and each cookie `j` has a size `s[j]`. If `s[j] >= g[i]`, we can assign
/// the cookie `j` to the child `i`, and the child `i` will be content. Your goal is to maximize
/// the number of your content children and output the maximum number.
struct Solution;

impl Solution {

    pub fn find_content_children(g: Vec<i32>, s: Vec<i32>) -> i32 {
        let mut children = BinaryHeap::new();
        for item in g {
            children.push(item);
        }

        let mut cookies = BinaryHeap::new();
        for size in s {
            cookies.push(size);
        }

        let mut result = 0;

        while !children.is_empty() && !cookies.is_empty() {
            let greed = children.pop().unwrap();
            let cookie = cookies.pop().unwrap();

            if greed <= cookie {
                result += 1;
            } else {
                cookies.push(cookie);
            }
        }

        result
    }

}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn example_1() {
        let g = vec![1,2,3];
        let s = vec![1,1];
        let result = Solution::find_content_children(g, s);
        assert_eq!(result, 1);
    }

    #[test]
    fn example_2() {
        let g = vec![1,2];
        let s = vec![1,2,3];
        let result = Solution::find_content_children(g, s);
        assert_eq!(result, 2);
    }
    

}
