/// Given two integer arrays `pushed` and `popped` each with distinct values, return `true` if this
/// could have been the result of a sequence of push and pop operations on an initially empty
/// stack, or `false` otherwise.
struct Solution;

impl Solution {

    // pushed = vec![1,2,3,4,5];
    // popped = vec![4,3,5,2,1];
    //
    // pop_index = 0
    // i = 0
    // item = 1
    // stack = { 1 }
    // 
    // i = 1
    // item = 2
    // stack = { 1, 2 }
    //
    // i = 2
    // item = 3
    // stack = { 1, 2, 3 }
    //
    // i = 3
    // item = 4
    // stack = { 1, 2, 3, 4 }
    // pop()
    // stack = { 1, 2, 3 }
    //
    // i = 4
    // item = 5
    // stack = { 1, 2, 3, 5 }
    // pop()
    // pop()
    // pop()
    // pop()
    pub fn validate_stack_sequences(pushed: Vec<i32>, popped: Vec<i32>) -> bool {
        let n = pushed.len();

        let mut stack = Vec::new();
        let mut pop_index = 0;

        for i in 0..n {
            let item = pushed[i];
            stack.push(item);
            while !stack.is_empty() && pop_index < n {
                let top = stack.last().copied().unwrap();
                if top == popped[pop_index] {
                    stack.pop();
                    pop_index += 1;
                } else {
                    break;
                }
            }
        }

        stack.is_empty()
    }

}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn example_1() {
        let pushed = vec![1,2,3,4,5];
        let popped = vec![4,5,3,2,1];
        let result = Solution::validate_stack_sequences(pushed, popped);
        assert!(result);
    }

    #[test]
    fn example_2() {
        let pushed = vec![1,2,3,4,5];
        let popped = vec![4,3,5,1,2];
        let result = Solution::validate_stack_sequences(pushed, popped);
        assert!(!result);
    }

}

// 1. Write down the problem ✓
// 2. Clarify the problem space ✓
// ** Input: pushed: Integer array
// ** Input: popped: Integer array
// ** Output: bool - could have been the result of a sequence of push and pop operations.
// ** pushed len >= 1 and <= 1000
// ** pushed elements >= 0 and <= 1000
// ** all pushed elements are unique
// ** popped len == pushed len
// ** popped is a permutation of pushed. So stack should be empty at the end.
//
// 3. Write down the test cases ✓
// ** Input: pushed = [1,2,3,4,5] popped = [4,5,3,2,1]
// ** Output: true
// ** push 1, push 2, push 3, push 4, pop 4, push 5, pop 5, pop 3, pop 2 pop 1
//
// ** Input: pushed = [1,2,3,4,5] popped = [4,3,5,1,2]
// ** Output: false
// ** push 1, push 2, push 3, push 4, pop 4, pop 3, push 5, pop ... fails here (2 vs. 1)
//
// 4. Describe and write down the algorithm
// ** Initialize a stack
// ** While the item on top of the stack is not equal to the next item to pop, push the next
//    item on to the stack. 
//    ** If the top of the stack equals the next item to pop, pop the item from the stack and move 
//       forward in the list to pop.
//
// 5. Start coding ✓
// 6. Walk through test cases
