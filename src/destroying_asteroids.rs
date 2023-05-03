/// You are given an integer `mass`, which represents the original mass of a
/// planet. You are further given an integer array `asteroids`, where
/// `asteroids[i]` is the mass of the `ith` asteroid.
///
/// You can arrange for the planet to collide with the asteroids in any
/// arbitrary order. If the mass of the planet is greater than or equal to the
/// mass of the asteroid, the asteroid is destroyed and the planet gains the
/// mass of the asteroid. Otherwise the planet is destroyed.
///
/// Return `true` if all asteroids can be destroyed. Otherwise, return `false`.
struct Solution;

impl Solution {

    pub fn asteroids_destroyed(mass: i32, asteroids: Vec<i32>) -> bool {
        let mut mass = mass as i64;
        let mut by_smallest = asteroids;
        by_smallest.sort();
        let mut result = true;

        for asteroid in by_smallest {
            let asteroid = asteroid as i64;
            if asteroid <= mass {
                mass += asteroid;
            } else {
                result = false;
                break;
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
        let mass = 10;
        let asteroids = vec![3,9,19,5,21];
        let result = Solution::asteroids_destroyed(mass, asteroids);
        assert!(result);
    }

    #[test]
    fn example_2() {
        let mass = 5;
        let asteroids = vec![4,9,23,4];
        let result = Solution::asteroids_destroyed(mass, asteroids);
        assert!(!result);
    }

}
