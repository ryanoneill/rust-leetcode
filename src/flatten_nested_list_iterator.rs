#[derive(Debug, PartialEq, Eq)]
pub enum NestedInteger {
    Int(i32),
    List(Vec<NestedInteger>),
}

/// You are given a nested list of integers `nestedList`. Each element is either an integer or a
/// list whose elements may also be integers or other lists. Implement an iterator to flatten it.
///
/// Implement the `NestedIterator` class:
///
/// * `NestedIterator(List<NestedInteger> nestedList)` Initializes the iterator with the nested
///   list `nestedList`.
///
/// * `int next()` Returns the next integer in the nested list.
///
/// * `boolean hasNext()` Returns `true` if there are still some integers in the nested list and
///   `false` otherwise.
struct NestedIterator {
    items: Vec<i32>,
    index: usize,
}

impl NestedIterator {

    #[allow(non_snake_case)] // This is how it's named on LeetCode.
    pub fn new(nestedList: Vec<NestedInteger>) -> Self {
        let mut items = Vec::new();
        Self::worker(&mut items, nestedList);
        Self { items, index: 0 }
    }

    fn worker(items: &mut Vec<i32>, list: Vec<NestedInteger>) {
        for item in list {
            match item {
                NestedInteger::Int(value) => {
                    items.push(value);
                }
                NestedInteger::List(sublist) => {
                    Self::worker(items, sublist);
                }
            }
        }
    }


    fn next(&mut self) -> i32 {
        let result = self.items[self.index];
        self.index += 1;
        result
    }

    fn has_next(&self) -> bool {
        let n = self.items.len();
        self.index < n
    }

}

#[cfg(test)]
mod tests {
    use super::NestedInteger;
    use super::NestedIterator;

    #[test]
    fn example_1() {
        let nested_list = vec![
            NestedInteger::List(vec![NestedInteger::Int(1), NestedInteger::Int(1)]),
            NestedInteger::Int(2),
            NestedInteger::List(vec![NestedInteger::Int(1), NestedInteger::Int(1)]),
        ];
        let mut iterator = NestedIterator::new(nested_list);
        let mut results = Vec::new();
        while iterator.has_next() {
            results.push(iterator.next());
        }
        assert_eq!(results, vec![1,1,2,1,1]);
    }

    #[test]
    fn example_2() {
        let nested_list = vec![
            NestedInteger::Int(1),
            NestedInteger::List(vec![
                NestedInteger::Int(4),
                NestedInteger::List(vec![
                    NestedInteger::Int(6)
                ])
            ])
        ];
        let mut iterator = NestedIterator::new(nested_list);
        let mut results = Vec::new();
        while iterator.has_next() {
            results.push(iterator.next());
        }
        assert_eq!(results, vec![1,4,6]);
    }

}
