use std::collections::HashSet;
use std::collections::VecDeque;

#[derive(Copy, Clone, Debug, PartialOrd, Ord, PartialEq, Eq, Hash)]
enum GeneChoice {
    A,
    C,
    G,
    T,
}

/// A gene string can be represented by an 8-character long string, with choices from `'A'`, `'C'`,
/// `'G'`, and `'T'`.
///
/// Suppose we need to investigate a mutation from a gene string `startGene` to a gene string
/// `endGene` where one mutation is defined as one single character changed in the gene string.
///
/// * For example, `"AACCGGTT" --> "AACCGGTA"` is one mutation.
///
/// There is also a gene bank `bank` that records all the valid gene mutations. A gene must be in
/// `bank` to make it a valid gene string.
///
/// Given the two gene strings `startGene` and `endGene` and the gene bank `bank`, return the
/// minimum number of mutations needed to mutate from `startGene` to `endGene`. If there is no such
/// a mutation, return `-1`.
///
/// Note that the starting point is assumed to be valid, so it might not be included in the bank.
struct Solution;

impl Solution {

    fn gene_string_to_gene_vec(gene_string: String) -> Vec<GeneChoice> {
        gene_string
            .chars()
            .map(|c| match c {
                'A' => GeneChoice::A,
                'C' => GeneChoice::C,
                'G' => GeneChoice::G,
                'T' => GeneChoice::T,
                _   => unreachable!("Not a valid gene string"),
            })
            .collect()
    }

    fn distance(gene1: &Vec<GeneChoice>, gene2: &Vec<GeneChoice>) -> usize {
        let mut result = 0;
        let n = gene1.len();
        for i in 0..n {
            let value1 = gene1[i];
            let value2 = gene2[i];
            if value1 != value2 {
                result += 1;
            }
        }

        result
    }

    pub fn min_mutation(start_gene: String, end_gene: String, bank: Vec<String>) -> i32 {
        let start_gene = Self::gene_string_to_gene_vec(start_gene);
        let end_gene = Self::gene_string_to_gene_vec(end_gene);
        let bank: Vec<Vec<GeneChoice>> = bank.into_iter().map(Self::gene_string_to_gene_vec).collect();

        let mut result = 0;
        let mut finished = false;
        let mut queue = VecDeque::new();
        let mut seen = HashSet::new();
        seen.insert(start_gene.clone());
        queue.push_back(start_gene);

        while !queue.is_empty() {
            let n = queue.len();
            for _ in 0..n {
                let gene = queue.pop_front().unwrap();
                if gene == end_gene {
                    finished = true;
                    break;
                } else {
                    for banked in &bank {
                        if !seen.contains(banked) && Self::distance(&gene, banked) == 1 {
                            seen.insert(banked.clone());
                            queue.push_back(banked.clone());
                        }
                    }
                }
            }
            if finished {
                break;
            } else {
                result += 1;
            }
        }

        if finished { result } else { -1 }
    }

}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn example_1() {
        let start_gene = "AACCGGTT".to_string();
        let end_gene = "AACCGGTA".to_string();
        let bank = vec![
            "AACCGGTA".to_string()
        ];
        let result = Solution::min_mutation(start_gene, end_gene, bank);
        assert_eq!(result, 1);
    }

    #[test]
    fn example_2() {
        let start_gene = "AACCGGTT".to_string();
        let end_gene = "AAACGGTA".to_string();
        let bank = vec![
            "AACCGGTA".to_string(),
            "AACCGCTA".to_string(),
            "AAACGGTA".to_string(),
        ];
        let result = Solution::min_mutation(start_gene, end_gene, bank);
        assert_eq!(result, 2);
    }

}
