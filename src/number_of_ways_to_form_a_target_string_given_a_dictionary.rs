use std::collections::HashMap;

/// You are given a list of strings of the same length `words` and a string `target`.
///
/// Your task is to form `target` using the given `words` under the following rules:
///
/// * `target` should be formed from left to right.
///
/// * To form the `ith` character (0-indexed) of `target`, you can choose the `kth` character of
///   the `jth` string in `words` if `target[i] = words[j][k]`.
///
/// * Once you use the `kth` character of the `jth` string of `words`, you can no longer use the
///   `xth` character of any string in `words` where `x <= k`. In other words, all characters to the
///   left of or at index `k` become unusable for every string.
///
/// * Repeat the process until you form the string `target`.
///
/// Notice that you can use multiple characters from the same string in `words` provided the
/// conditions above are met.
///
/// Return the number of ways to form `target` from `words`. Since the answer may be too large,
/// return it modulo 10^9 + 7.
struct Solution;

impl Solution {

    fn build_words_data(words: Vec<String>) -> Vec<Vec<i64>> {
        let mut result = vec![];
        for word in words.into_iter() {
            let mut i = 0;
            for letter in word.chars() {
                if result.len() <= i {
                    result.push(vec![0; 26]);
                }
                let j = (letter as usize) - ('a' as usize);
                result[i][j] += 1;
                i += 1;
            }
        }
        result
    }

    fn worker(
        values: &mut HashMap<(usize, usize), i64>, 
        words_data: &Vec<Vec<i64>>,
        target: &Vec<char>,
        w_index: usize,
        t_index: usize,
    ) -> i64 {
        let w_len = words_data.len();
        let t_len = target.len();

        let key = (w_index, t_index);
        if values.contains_key(&key) {
            values[&key]
        } else if w_index >= w_len {
            0
        } else if t_index >= t_len {
            0
        } else {
            let mut result = 0;

            let l_index = target[t_index] as usize - 'a' as usize;
            let count = words_data[w_index][l_index];
            if count > 0 {
                if t_index == t_len - 1 {
                    result += count;
                } else {
                    result += count * Self::worker(values, words_data, target, w_index + 1, t_index + 1);
                }
            }
            result += Self::worker(values, words_data, target, w_index + 1, t_index);
            result = result % 1000000007;

            values.insert(key, result);
            result
        }
    }

    pub fn num_ways(words: Vec<String>, target: String) -> i32 {
        let words_data = Self::build_words_data(words);
        let letters: Vec<char> = target.chars().collect();

        let mut values = HashMap::new();

        let mut result = Self::worker(&mut values, &words_data, &letters, 0, 0);
        result = result % 1000000007;
        result as i32
    }

}

#[cfg(test)] 
mod tests {
    use super::Solution;

    #[test]
    fn example_1() {
        let words = vec![str!("acca"), str!("bbbb"), str!("caca")];
        let target = str!("aba");
        let result = Solution::num_ways(words, target);
        assert_eq!(result, 6);
    }

    #[test]
    fn example_2() {
        let words = vec![str!("abba"), str!("baab")];
        let target = str!("bab");
        let result = Solution::num_ways(words, target);
        assert_eq!(result, 4);
    }

    #[test]
    fn example_3() {
        let words = vec!["cabbaacaaaccaabbbbaccacbabbbcb","bbcabcbcccbcacbbbaacacaaabbbac","cbabcaacbcaaabbcbaabaababbacbc","aacabbbcaaccaabbaccacabccaacca","bbabbaabcaabccbbabccaaccbabcab","bcaccbbaaccaabcbabbacaccbbcbbb","cbbcbcaaaacacabbbabacbaabbabaa","cbbbbbbcccbabbacacacacccbbccca","bcbccbccacccacaababcbcbbacbbbc","ccacaabaaabbbacacbacbaaacbcaca","bacaaaabaabccbcbbaacacccabbbcb","bcbcbcabbccabacbcbcaccacbcaaab","babbbcccbbbbbaabbbacbbaabaabcc","baaaacaaacbbaacccababbaacccbcb","babbaaabaaccaabacbbbacbcbababa","cbacacbacaaacbaaaabacbbccccaca","bcbcaccaabacaacaaaccaabbcacaaa","cccbabccaabbcbccbbabaaacbacaaa","bbbcabacbbcabcbcaaccbcacacccca","ccccbbaababacbabcaacabaccbabaa","caaabccbcaaccabbcbcaacccbcacba","cccbcaacbabaacbaaabbbbcbbbbcbb","cababbcacbabcbaababcbcabbaabba","aaaacacaaccbacacbbbbccaabcccca","cbcaaaaabacbacaccbcbcbccaabaac","bcbbccbabaccabcccacbbaacbbcbba","cccbabbbcbbabccbbabbbbcaaccaab","acccacccaabbcaccbcaaccbababacc","bcacabaacccbbcbbacabbbbbcaaaab","cacccaacbcbccbabccabbcbabbcacc","aacabbabcaacbaaacaabcabcaccaab","cccacabacbabccbccaaaaabbcacbcc","cabaacacacaaabaacaabababccbaaa","caabaccaacccbaabcacbcbbabccabc","bcbbccbbaaacbaacbccbcbababcacb","bbabbcabcbbcababbbbccabaaccbca","cacbbbccabaaaaccacbcbabaabbcba","ccbcacbabababbbcbcabbcccaccbca","acccabcacbcbbcbccaccaacbabcaab","ccacaabcbbaabaaccbabcbacaaabaa","cbabbbbcabbbbcbccabaabccaccaca","acbbbbbccabacabcbbabcaacbbaacc","baaababbcabcacbbcbabacbcbaaabc","cabbcabcbbacaaaaacbcbbcacaccac"];
        let words: Vec<String> = words.into_iter().map(|w| w.to_string()).collect();
        let target = str!("acbaccacbbaaabbbabac");
        let result = Solution::num_ways(words, target);
        assert_eq!(result, 555996654);
    }

    #[test]
    fn example_4() {
        let words = vec!["ccbdddbcabddcbacbacdbcabacbdcddcadbdbcacacccabddaba","ccdaccbbadbcabcbabcbcbccdaadabdbdaccddadcddbbbddcbc","cabacbbdbbbccbdcababcccaaadacbbdbaacdccaadbdadbdacc","bacabbadddbdcbcaabaababaaddaddcdbcbaddbcaabcbddccdd","accbcccdacccdadbdaacbadbbdbddaddddddacbbacbbddaddad","babdaaddaabcdaaccacaabdccaccdaacbbbdaaacbcbbcdcadab","dccddbcaddcddacdaccdbaadccabcdddbaaacbdacccdbabbcbc","cddadaabbcaccbcbbbbaacbdcbdacbbcddcccccbbaaaaadcaac","acbbcdaddaadaaddbdccdaddbdddbbadcbabbbadbacbddbbaba","daaabcdcccddbcdbdacbbbbbbcddccbdabdadbdadaabbbaaddc","acdadcacdacdbaacbccdcbbbddadaccdbbcadcadcdaabbcdccd","acaccddcaabbbbdababdababccbbbbbddcaadbaadaaabddabbc","dcbaaacbbcacabcbdcdbaadddbacdacccdcaccdadbdccbddadb","cdaddababcbabbbbbdadcccadacddadadddddddabcbcdcacadb","dabddcdbdaacdddbbbabbddcbddacaadcbdaadcaddaccbbbaaa","dddacacbacbdaabdccacbaccbabbdaadccacaddadbddcbbdaab","abbcbaacaddbccbbaaddbdbadcccaaadddddcacddbaabacccdc","bdaabdcabddaadbbcadacdbdbdaaacccadddadbcacbacbcdadd","daaaddddccbadccddcccbccccababdaabdaacccaacacaccbcbc","cbddabcaacbdacabdababbabbdbcccbcdadcacdcabadcbbcbac","bbdabbddadbdaabdadcbddbdddcccdaaaaacabdabbbbaabdadd","bacacdaabbdcdbbbcccdddcabaadaacddabaabdaddababbcaba","ddabdcdabdbaaabbacacdbababddcadacdbdcbccdbdddadacba","dbbbacdbbaacbbccaaaaabdcbdacaadaaabbbccccdbddadcddc","ccbcacaadbdcabbabdddbcaabacaabccbcdccbaccbacadbacda","bbbaccdbaaaccbaabbccdcbbacbbdcdcaaddbdacaabdbcaadab","dcabdcdbdbdcaabbbaaadbccacbcaaabdabbbdaccddddbbdbaa","bccbcbaaccbcadccaacdbcdccdbaacdcccaacbddbabcadccbba","dacdcdddadbdcbaacdaddaabbbcbdaabdadcbacadadabdbddda","dbadaddbaaaccacdadbbacaabbbdadaaabdbaacadacdbbacacc","ddccaddbdbdccbcaddbbabdadbbdbccbbddbdacacbacadddcab","dcbaabccbdcaaadccadbbdccbcddddadcabddaadcddddadaabc","cbaddbaccccbabbbcccbcdaccbacbdadcddaaaabdbddbbcadcb","bbbbdbdcbacdddabbbcbaccadabbcdcccdcbacdddcdbaccaddd","aabacbbccdcacdaacacbacccbbdddaaccbacdbdaacdbadddcbc","bcdbddbabacaacbacbaddccaadcdbabddadadadabcacbdcddbd","adcccdcddadccadbdbadcbdcdbaddcccadbabdadabcdcbdaacc","accbadbaabbdcdbdbcbbdcbadbccdaccbacdbdddbbbaccacdba","cabbccbaaddddaabbdccaadccaaddcaababbbdcddcbdcddadab","abadcbaaadacaddccdbdbbdcbcdaccabbbbcbcdcccddbcbbacc","bdddaadadbacdddcabcbaddadacddabcdbbacababcdbaabaccb","dabbaaaaaacabdcdcdcbdccdaababbcdabbdadadadacaadddca","dcddacdaddadbbcadbdddbdabbdbdadcbadddbadabcadadacac","badbcadcbbdaaaccbdcabdbbcaadabccbcacabacdbbcbbbdbac","bacabcadcccacabcbccbacdcddcbbdbccaadcaccadaacbdcddd","acccddcabddbdbcaadbdabdbaacdaddddddaaccddabbdcdaabd","cbbccaaacbbbcaccadcbdcbabadcacdccbcabbbcaaaabcadbda","cbbcbcccddccaccabdbddcdbcdccbdbbaabdbbacbbbaaaccdba","bcdbadcdabbabaaacddddabddaadccdddabdbbdaabdaaacdbba","ddaabbaddccacbbabbadccbcdacddcbaccaccdcdaabbdbaaadb","dcaacaabbaacacababdcbbbdadcbddbdcbdddaaaabdaacbcdac","dddcbcbcacdcaacbccaccccaadcdcccaabdcbadbbbabaccccda","cddcdaadbbadbdaadaacdbddbacdaccdaddccddbbcbbddcdcca","abbdaccabaddcacddacabdadaccaadacabacdddbcddacbccccd","bcbaadbbdbaccabbdaaaadbbccbbacaccbbaaaadbdcbbdddbdc","adbbacaaabbcaddaddcaddbdabbbbabbddcadbdaaddddbbccda","bccbdddddadcbabdbcaacabcaadcddacaabbaddcaaabddbabcc","bdbabaabdcbddaccbcdddaacdbdabddabbaddbcaabcabbddcbc","bcacbbdcdbabcbdbdadddadcaacbcabaccbcdaccdaadabaccba","baccdbabcbacdadbddaabbdadbcdccdabbdbdcdacdccddbcddd","aaccbbbabddbdbdabcabdacdcdccdcccbcdbdbcaadcbabdacca","cbcccccbcaabbddbcaadccdddaaacdcddccdbdcdbcadbdccccc","cccbddbacccdadbcdbcbbddabcbbbbcbdccaacbaaaddabccbdd","acbabbbadcbcdbbabaadaadacddacdbccddcddccddbacaadbbc","bbdccabcdcdddcbddacaababddaaabdbacbccbcbbccbccacbdb","cabadddbbbbccbcccadccbbccabdadbdccdacddacccbadddddb","bbadadcbaaddbbcdcdaccdddacddccdadccbadbcacbabccabbd","cdbaabdaddacadbbcdbadcaadbdabacadbabdbbcbbabacccaaa","ddadbdacdcabcdcabacbcdcbdccbaddbdbdacdbdabaabbdadcb","acacbbababddadabbcbacabddcadbccdcacdbbbbdcbdaddbdcb","aadcbacaadbabbcccbbacccccabdabbbbcdbbbbcdbabccadcdc","cbadbbabbdccdacbcaabbaccadabcdaabdcbbbccabccaaabadc","dddadabbacaacaccddcdbdcadabbbbbacacabcaaabaaacbbbad","ccccbdbcbdabbbadbacacbdbabbcabaaadacdbbdcccdaacaaca","badcacccbacaccdaadcbbcddcbaddbbcbdaaaccadacccabacac","dcabdaaccdacaacbdbdddaacaddaaddcccdaadabcdbadaabadb","cbddddcbbdacbbcbabdccadadaaabadadbacbbbaaabccaddcca"]; 
        let words: Vec<String> = words.into_iter().map(|w| w.to_string()).collect();
        let target = str!("ccbabdbbbddbdccdacabdabdabbcda");
        let result = Solution::num_ways(words, target);
        assert_eq!(result, 99247761);
    }

}
