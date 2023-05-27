use std::collections::HashMap;
use std::collections::HashSet;
use std::collections::VecDeque;

#[derive(Clone, PartialEq, PartialOrd)]
struct Relationship {
    top: String,
    bottom: String,
    value: f64,
}

impl Relationship {

    pub fn new(top: String, bottom: String, value: f64) -> Self {
        Relationship { top, bottom, value }
    }

}

/// You are given an array of variable pairs `equations` and an array of real
/// numbers `values`, where `equations[i] = [Ai, Bi]` and `values[i]` represent
/// the equation `Ai / Bi = values[i]`. Each `Ai` or `Bi` is a string
/// that represents a single variable. 
///
/// You are also given some `queries`, where `queries[j] = [Cj, Dj]` represents
/// the `jth` query where you must find the answer for `Cj / Dj = ?`.
///
/// Return the answers to all queries. If a single answer cannot be determined,
/// return `-1.0`.
///
/// Note: The input is always valid. You may assume that evaluating the queries
/// will not result in division by zero and that there is no contradiction.
struct Solution;

impl Solution {

    fn build_relationships(equations: Vec<Vec<String>>, values: Vec<f64>) -> HashMap<String, Vec<Relationship>> {
        let n = equations.len();
        let mut relationships = HashMap::new();
        for i in 0..n {
            let equation = &equations[i];
            let top = equation[0].clone();
            let bottom = equation[1].clone();
            let value = values[i];
            let relationship1 = Relationship::new(top.clone(), bottom.clone(), value);
            let relationship2 = Relationship::new(bottom.clone(), top.clone(), 1.0 / value);

            relationships
                .entry(top)
                .or_insert(Vec::new())
                .push(relationship1);
            relationships
                .entry(bottom)
                .or_insert(Vec::new())
                .push(relationship2);

        }

        relationships
    }

    fn evaluate_query(relationships: &HashMap<String, Vec<Relationship>>, query: &Vec<String>) -> f64 {
        let mut seen = HashSet::new();
        let mut result = -1.0;
        let from = query[0].clone();
        let to = query[1].clone();

        let mut queue = VecDeque::new();
        seen.insert(from.clone());
        if relationships.contains_key(&from) {
            if from == to {
                result = 1.0;
            } else if !relationships.contains_key(&to) {
                result = -1.0;
            } else {
                let rels = &relationships[&from];
                for rel in rels {
                    queue.push_back(rel.clone());
                    seen.insert(rel.bottom.clone());
                }
            }
        }
        while !queue.is_empty() {
            let rel = queue.pop_front().unwrap();
            if rel.bottom == to {
                result = rel.value;
                break;
            } else if relationships.contains_key(&rel.bottom) {
                let rels = &relationships[&rel.bottom];
                for next_rel in rels {
                    if !seen.contains(&next_rel.bottom) {
                        seen.insert(next_rel.bottom.clone());
                        let top = rel.top.clone();
                        let bottom = next_rel.bottom.clone();
                        let value = rel.value * next_rel.value;
                        let new_rel = Relationship::new(top, bottom, value);
                        queue.push_back(new_rel);
                    }
                }
            }
        }

        result
    }

    pub fn calc_equation(equations: Vec<Vec<String>>, values: Vec<f64>, queries: Vec<Vec<String>>) -> Vec<f64> {
        let relationships = Self::build_relationships(equations, values);
        queries
            .iter()
            .map(|query| Self::evaluate_query(&relationships, query))
            .collect()
    }

}

#[cfg(test)]
mod tests {
    use crate::equal_row_and_column_pairs;

    use super::Solution;

    #[test]
    fn example_1() {
        let equations = vec![
            vec!["a".to_string(), "b".to_string()],
            vec!["b".to_string(), "c".to_string()]
        ];
        let values = vec![2.0,3.0];
        let queries = vec![
            vec!["a".to_string(), "c".to_string()],
            vec!["b".to_string(), "a".to_string()],
            vec!["a".to_string(), "e".to_string()],
            vec!["a".to_string(), "a".to_string()],
            vec!["x".to_string(), "x".to_string()]
        ];
        let result = Solution::calc_equation(equations, values, queries);
        assert_eq!(result, vec![6.0,0.5,-1.0,1.0,-1.0]);
    }

    #[test]
    fn example_2() {
        let equations = vec![
            vec!["a".to_string(), "b".to_string()],
            vec!["b".to_string(), "c".to_string()],
            vec!["bc".to_string(), "cd".to_string()]
        ];
        let values = vec![1.5,2.5,5.0];
        let queries = vec![
            vec!["a".to_string(), "c".to_string()],
            vec!["c".to_string(), "b".to_string()],
            vec!["bc".to_string(), "cd".to_string()],
            vec!["cd".to_string(), "bc".to_string()]
        ];
        let result = Solution::calc_equation(equations, values, queries);
        assert_eq!(result, vec![3.75,0.4,5.0,0.2]);
    }

    #[test]
    fn example_3() {
        let equations = vec![
            vec!["a".to_string(), "b".to_string()]
        ];
        let values = vec![0.5];
        let queries = vec![
            vec!["a".to_string(), "b".to_string()],
            vec!["b".to_string(), "a".to_string()],
            vec!["a".to_string(), "c".to_string()],
            vec!["x".to_string(), "y".to_string()]
        ];
        let result = Solution::calc_equation(equations, values, queries);
        assert_eq!(result, vec![0.5,2.0,-1.0,-1.0]);
    }

}
