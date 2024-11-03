/// Given an array of unique integers `preorder`, return `true` if it is the correct preorder
/// traversal sequence of a binary search tree.
struct Solution;

impl Solution {

    pub fn verify_preorder(preorder: Vec<i32>) -> bool {
        let mut result = true;
        let mut stack: Vec<i32> = Vec::new();
        let mut last = i32::MIN;

        for num in preorder {
            loop {
                if stack.is_empty() {
                    break;
                } else {
                    let s = stack.len();
                    let peek = stack[s-1];
                    if peek < num {
                        last = peek;
                        stack.pop();
                    } else {
                        break;
                    }
                }
            }
            if num <= last {
                result = false;
                break;
            }
            stack.push(num);
        }

        result
    }

}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn example_1() {
        let preorder = vec![5,2,1,3,6];
        let result = Solution::verify_preorder(preorder);
        assert!(result);
    }

    #[test]
    fn example_2() {
        let preorder = vec![5,2,6,1,3];
        let result = Solution::verify_preorder(preorder);
        assert!(!result);
    }


}
