use std::collections::HashMap;

/// You are given an array of strings `products` and a string `searchWord`. 
///
/// Design a system that suggests at most three product names from `products` after each character
/// of `searchWord` is typed. Suggested products should have common prefix with `searchWord`. If
/// there are more than three products with a common prefix return the three lexicographically
/// minimums products.
///
/// Return a list of lists of the suggested products after each character of `searchWord` is typed.
struct Solution;

impl Solution {

    pub fn suggested_products(products: Vec<String>, search_word: String) -> Vec<Vec<String>> {
        let mut prefixes: HashMap<String, Vec<String>> = HashMap::new();
        let mut products = products;
        products.sort_unstable();

        for product in &products {
            let chars: Vec<char> = product.chars().collect();
            let n = chars.len();
            let mut prefix = String::from("");
            for i in 0..n {
                prefix.push(chars[i]);
                prefixes
                    .entry(prefix.clone())
                    .and_modify(|words| {
                        if words.len() < 3 {
                            words.push(product.to_owned());
                        }
                    })
                    .or_insert(vec![product.to_owned()]);
            }
        }

        let letters: Vec<char> = search_word.chars().collect();
        let n = letters.len();
        let mut results = Vec::with_capacity(n);
        let mut prefix = String::from("");

        for i in 0..n {
            prefix.push(letters[i]);

            if prefixes.contains_key(&prefix) {
                results.push(prefixes[&prefix].clone());
            } else {
                results.push(vec![]);
            }
        }

        results
    }

}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn example_1() {
        let products = vec![str!("mobile"), str!("mouse"), str!("moneypot"), str!("monitor"), str!("mousepad")];
        let search_word = str!("mouse");
        let results = Solution::suggested_products(products, search_word);
        assert_eq!(
            results,
            vec![
                vec![str!("mobile"), str!("moneypot"), str!("monitor")],
                vec![str!("mobile"), str!("moneypot"), str!("monitor")],
                vec![str!("mouse"), str!("mousepad")],
                vec![str!("mouse"), str!("mousepad")],
                vec![str!("mouse"), str!("mousepad")],
            ]);
    }

    #[test]
    fn example_2() {
        let products = vec![str!("havana")];
        let search_word = str!("havana");
        let results = Solution::suggested_products(products, search_word);
        assert_eq!(
            results,
            vec![
                vec![str!("havana")],
                vec![str!("havana")],
                vec![str!("havana")],
                vec![str!("havana")],
                vec![str!("havana")],
                vec![str!("havana")],
            ]);
    }

}
