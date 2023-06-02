use std::collections::HashMap;
use std::collections::HashSet;
use std::collections::VecDeque;

#[derive(Copy, Clone, Debug, PartialOrd, Ord, PartialEq, Eq, Hash)]
struct Bomb {
    index: usize,
    x: i32,
    y: i32,
    radius: i32,
}

impl Bomb {

    fn new(index: usize, x: i32, y: i32, radius: i32) -> Self {
        Self { index, x, y, radius }
    }

    fn from_vec(index: usize, values: Vec<i32>) -> Self {
        Self::new(index, values[0], values[1], values[2])
    }

    fn within_range(&self, bomb: &Bomb) -> bool {
        let diff_x = (bomb.x - self.x) as i64;
        let diff_y = (bomb.y - self.y) as i64;
        let distance = ((diff_x.pow(2) + diff_y.pow(2)) as f64).sqrt();
        distance <= self.radius as f64
    }

}

/// You are given a list of bombs. The range of a bomb is defined as the area
/// where its effect can be felt. This area is in the shape of a circle with
/// the center as the location of the bomb.
///
/// The bombs are represented by a 0-indexed 2D integer array `bombs` where
/// `bombs[i] = [xi, yi, ri]`. `xi` and `yi` denote the X-coordinate and
/// Y-coordinate of the location of the `ith` bomb, whereas `ri` denotes the
/// radius of its range.
///
/// You may choose to detonate a single bomb. When a bomb is detonated, it
/// will detonate all bombs that lie in its range. These bombs will further
/// detonate bombs that lie in their ranges.
///
/// Given the list of `bombs`, return the maximum number of bombs that can be
/// detonated if you are allowed to detonate only one bomb.
struct Solution;

impl Solution {

    fn find_within_range(bombs: &Vec<Bomb>) -> HashMap<usize, HashSet<usize>> {
        let mut result = HashMap::new();

        for detonated in bombs {
            for affected in bombs {
                if detonated != affected && detonated.within_range(affected) {
                    result
                        .entry(detonated.index)
                        .or_insert(HashSet::new())
                        .insert(affected.index);
                }
            }
        }

        result
    }

    fn build_bombs(bombs: Vec<Vec<i32>>) -> Vec<Bomb> {
        let n = bombs.len();
        let mut result = Vec::with_capacity(n);
        for (i, values) in bombs.into_iter().enumerate() {
            let bomb = Bomb::from_vec(i, values);
            result.push(bomb);
        }
        result
    }

    fn start_with(saved: &mut HashMap<usize, HashSet<usize>>, within_range: &HashMap<usize, HashSet<usize>>, starter: usize) -> HashSet<usize> {
        let mut seen: HashSet<usize> = HashSet::new();
        let mut queue: VecDeque<usize> = VecDeque::new();

        seen.insert(starter);
        queue.push_back(starter);
        while !queue.is_empty() {
            let bomb = queue.pop_front().unwrap();
            if within_range.contains_key(&bomb) {
                let affected_bombs = &within_range[&bomb];
                for affected in affected_bombs {
                    if !seen.contains(&affected) {
                        if saved.contains_key(affected) {
                            let known = &saved[affected];
                            seen = seen.union(known).copied().collect();
                        } else {
                            seen.insert(*affected);
                            queue.push_back(*affected);
                        }
                    }
                }
            }
        }

        seen
    }

    pub fn maximum_detonation(bombs: Vec<Vec<i32>>) -> i32 {
        let bombs: Vec<Bomb> = Self::build_bombs(bombs);
        let within_range = Self::find_within_range(&bombs);
        let mut result = 0;
        let mut saved = HashMap::new();

        for bomb in &bombs {
            let bomb_index = bomb.index;
            let all_affected = Self::start_with(&mut saved, &within_range, bomb_index);
            result = result.max(all_affected.len());
            saved.insert(bomb_index, all_affected);
        }

        result as i32
    }

}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn example_1() {
        let bombs = vec![vec![2,1,3], vec![6,1,4]];
        let result = Solution::maximum_detonation(bombs);
        assert_eq!(result, 2);
    }

    #[test]
    fn example_2() {
        let bombs = vec![vec![1,1,5], vec![10,10,5]];
        let result = Solution::maximum_detonation(bombs);
        assert_eq!(result, 1);
    }

    #[test]
    fn example_3() {
        let bombs = vec![vec![1,2,3], vec![2,3,1], vec![3,4,2], vec![4,5,3], vec![5,6,4]];
        let result = Solution::maximum_detonation(bombs);
        assert_eq!(result, 5);
    }

    #[test]
    fn same_structure() {
        let bombs = vec![vec![4,4,3], vec![4,4,3]];
        let result = Solution::maximum_detonation(bombs);
        assert_eq!(result, 2);
    }

    #[test]
    fn large_numbers() {
        let bombs = vec![
            vec![85024, 58997, 3532], vec![65196, 42043, 9739], vec![85872, 75029, 3117], 
            vec![73014, 91183, 7092], vec![29098, 40864, 7624], vec![11469, 13607, 4315], 
            vec![98722, 69681, 9656], vec![75140, 42250, 421], vec![92580, 44040, 4779], 
            vec![58474, 78273, 1047], vec![27683, 4203, 6186], vec![10714, 24238, 6243], 
            vec![60138, 81791, 3496], vec![16227, 92418, 5622], vec![60496, 64917, 2463], 
            vec![59241, 62074, 885], vec![11961, 163, 5815], vec![37757, 43214, 3402], 
            vec![21094, 98519, 1678], vec![49368, 22385, 1431], vec![6343, 53798, 159], 
            vec![80129, 9282, 5139], vec![69565, 32036, 6827], vec![59372, 64978, 6575], 
            vec![44948, 71199, 7095], vec![46390, 91701, 1667], vec![37144, 98691, 8128], 
            vec![13558, 81505, 4653], vec![41234, 48161, 9304], vec![14852, 3206, 5369]
        ];
        let result = Solution::maximum_detonation(bombs);
        assert_eq!(result, 3);
    }

}
