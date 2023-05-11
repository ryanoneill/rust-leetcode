/// We are given an array `asteroids` of integers representing asteroids in a
/// row.
///
/// For each asteroid, the absolute value represents its size, and the sign
/// represents its direction (positive meaning right, negative meaning left).
/// Each asteroid moves at the same speed.
///
/// Find out the state of the asteroids after all collisions. If two asteroids
/// meet, the smaller one will explode. If both are the same size, both will
/// explode. Two asteroids moving in the same direction will never meet.
struct Solution;

impl Solution {

    fn peek(stack: &Vec<i32>) -> Option<i32> {
        if stack.is_empty() { None }
        else {
            let n = stack.len();
            Some(stack[n-1])
        }
    }

    pub fn asteroid_collision(asteroids: Vec<i32>) -> Vec<i32> {
        let mut stack = Vec::new();

        for asteroid in asteroids {
            loop {
                match Self::peek(&stack) {
                    None => {
                        stack.push(asteroid);
                        break;
                    }
                    Some(other) => {
                        if asteroid > 0 {
                            stack.push(asteroid);
                            break;
                        } else if asteroid < 0 && other < 0 {
                            stack.push(asteroid);
                            break;
                        } else if asteroid.abs() > other.abs() {
                            stack.pop();
                        } else if asteroid.abs() < other.abs() {
                            break;
                        } else {
                            stack.pop();
                            break;
                        }
                    }
                }
            }
        }

        stack
    }

}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn example_1() {
        let asteroids = vec![5,10,-5];
        let result = Solution::asteroid_collision(asteroids);
        assert_eq!(result, vec![5,10]);
    }

    #[test]
    fn example_2() {
        let asteroids = vec![8, -8];
        let result = Solution::asteroid_collision(asteroids);
        assert_eq!(result, vec![]);
    }

    #[test]
    fn example_3() {
        let asteroids = vec![10, 2, -5];
        let result = Solution::asteroid_collision(asteroids);
        assert_eq!(result, vec![10]);
    }

    #[test]
    fn larger_example() {
        let asteroids = vec![2,4,-8,4,-2,-12];
        let result = Solution::asteroid_collision(asteroids);
        assert_eq!(result, vec![-8, -12]);
    }

}
