use std::ops::Index;

struct LowercaseLetterCounter {
    counts: Vec<usize>
}

impl LowercaseLetterCounter {

    pub fn new(s: String) -> Self {
        let mut counts = vec![0; 26];
        for c in s.chars() {
            let index = (c as usize) - ('a' as usize);
            counts[index] += 1;
        }
        Self {
            counts
        }
    }

    pub fn max(&self, other: &Self) -> Self {
        let mut counts = vec![0; 26];
        for i in 0..26 {
            counts[i] = self[i].max(other[i]);
        }
        Self {
            counts
        }
    }

}

impl Index<usize> for LowercaseLetterCounter {
    type Output = usize;

    fn index(&self, index: usize) -> &Self::Output {
        &self.counts[index]
    }

}

impl Index<char> for LowercaseLetterCounter {
    type Output = usize;

    fn index(&self, index: char) -> &Self::Output {
        let i = (index as usize) - ('a' as usize);
        &self.counts[i]
    }

}

#[cfg(test)]
mod tests {
    use super::LowercaseLetterCounter;

    #[test]
    fn example_1() {
        let s = str!("hello");
        let counts = LowercaseLetterCounter::new(s);
        assert_eq!(counts['a'], 0);
        assert_eq!(counts['l'], 2);
        assert_eq!(counts['h'], 1);
        assert_eq!(counts['r'], 0);
    }

    #[test]
    fn example_2() {
        let s = str!("warriors");
        let t = str!("lakers");
        let counts_s = LowercaseLetterCounter::new(s);
        let counts_t = LowercaseLetterCounter::new(t);
        let result = counts_s.max(&counts_t);
        assert_eq!(result['a'], 1);
        assert_eq!(result['b'], 0);
        assert_eq!(result['c'], 0);
        assert_eq!(result['d'], 0);
        assert_eq!(result['e'], 1);
        assert_eq!(result['f'], 0);
        assert_eq!(result['g'], 0);
        assert_eq!(result['h'], 0);
        assert_eq!(result['i'], 1);
        assert_eq!(result['j'], 0);
        assert_eq!(result['k'], 1);
        assert_eq!(result['l'], 1);
        assert_eq!(result['m'], 0);
        assert_eq!(result['n'], 0);
        assert_eq!(result['o'], 1);
        assert_eq!(result['p'], 0);
        assert_eq!(result['q'], 0);
        assert_eq!(result['r'], 3);
        assert_eq!(result['s'], 1);
        assert_eq!(result['t'], 0);
        assert_eq!(result['u'], 0);
        assert_eq!(result['v'], 0);
        assert_eq!(result['w'], 1);
        assert_eq!(result['x'], 0);
        assert_eq!(result['y'], 0);
        assert_eq!(result['z'], 0);
    }

}
