use std::collections::VecDeque;
use std::iter::FromIterator;

/// The school cafeteria offers circular and square sandwiches at lunch break, referred to by
/// numbers `0` and `1` respectively. All students stand in a queue. Each student either prefers
/// square or circular sandwiches.
///
/// The number of sandwiches in the cafeteria is equal to the number of students. The sandwiches
/// are placed in a stack. At each step:
///
/// * If the student at the front of the queue prefers the sandwich on the top of the stack, they
/// will take it and leave the queue.
///
/// * Otherwise, they will leave it and go to the queue's end.
///
/// This continues until none of the queue students want to take the top sandwich and are thus
/// unable to eat.
///
/// You are given two integer arrays `students` and `sandwiches` where `sandwiches[i]` is the type
/// of the `ith` sandwich in the stack (`i = 0` is the top of the stack) and `students[j]` is the
/// preference of the `jth` student in the initial queue (`j = 0` is the front of the queue).
/// Return the number of students that are unable to eat.
/// 
struct Solution;

impl Solution {

    pub fn count_students(students: Vec<i32>, sandwiches: Vec<i32>) -> i32 {
        let mut students: VecDeque<i32> = VecDeque::from_iter(students.into_iter());

        for sandwich in sandwiches {
            let mut n = students.len();
            while n > 0 {
                let student = students.pop_front().unwrap();
                if student == sandwich {
                    break;
                } else {
                    students.push_back(student);
                    n -= 1;
                }
            }
            if n == 0 {
                break;
            }
        }
        
        students.len() as i32
    }

}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn example_1() {
        let students = vec![1,1,0,0];
        let sandwiches = vec![0,1,0,1];
        let result = Solution::count_students(students, sandwiches);
        assert_eq!(result, 0);
    }

    #[test]
    fn example_2() {
        let students = vec![1,1,1,0,0,1];
        let sandwiches = vec![1,0,0,0,1,1];
        let result = Solution::count_students(students, sandwiches);
        assert_eq!(result, 3);
    }

}
