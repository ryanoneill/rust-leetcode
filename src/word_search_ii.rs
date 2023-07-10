use std::collections::HashMap;
use std::collections::HashSet;

struct DictionaryNode {
    value: char,
    items: HashMap<char, DictionaryNode>,
    ends_at: bool,
    word: String,
}

impl DictionaryNode {

    fn new(value: char) -> Self {
        Self { value, items: HashMap::new(), ends_at: false, word: String::from("") }
    }

}

struct Dictionary {
    root: DictionaryNode,
}

impl Dictionary {

    fn new() -> Self {
        Self { root: DictionaryNode::new('*') }
    }

    fn add_word(&mut self, word: String) {
        let mut current = &mut self.root;
        let letters: Vec<char> = word.chars().collect();
        for letter in letters {
            if !current.items.contains_key(&letter) {
                let node = DictionaryNode::new(letter);
                current.items.insert(letter, node);
            }
            current = current.items.get_mut(&letter).unwrap();

        }
        current.ends_at = true;
        current.word = word;
    }

    fn search_word(&self, letters: Vec<char>) -> bool {
        Self::search_word_worker(&self.root, &letters, 0)
    }

    fn search_word_worker(node: &DictionaryNode, letters: &Vec<char>, i: usize) -> bool {
        let n = letters.len();
        let letter = letters[i];
        if node.items.contains_key(&letter) {
            let next = &node.items[&letter];
            if i == n-1 {
                next.ends_at
            } else {
                Self::search_word_worker(next, letters, i+1)
            }
        } else {
            false
        }
    }

}

#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct Position {
    row: usize,
    col: usize,
}

impl Position {

    fn new(row: usize, col: usize) -> Self {
        Self { row, col }
    }

}

struct State<'a> {
    position: Position,
    node: &'a DictionaryNode,
    seen: HashSet<Position>,
}

impl<'a> State<'a> {

    fn new(position: Position, node: &'a DictionaryNode) -> Self {
        let mut seen = HashSet::new();
        seen.insert(position);
        Self { position, node, seen }
    }

    fn with_seen(position: Position, node: &'a DictionaryNode, seen: HashSet<Position>) -> Self {
        Self { position, node, seen }
    }

}

/// Given an `m x n` `board` of characters and a list of strings `words`, return all words on the
/// board.
///
/// Each word must be constructed from letters of sequentially adjacent cells, where adjacent cells
/// are horizontally or vertically neighboring. The same letter cell may not be used more than once
/// in a word.
struct Solution;

impl Solution {

    fn to_dictionary(words: Vec<String>) -> Dictionary {
        let mut result = Dictionary::new();
        for word in words {
            result.add_word(word);
        }
        result
    }

    fn find_words_from(
        board: &Vec<Vec<char>>,
        dictionary: &Dictionary,
        position: Position,
        result_set: &mut HashSet<String>
    ) {
        let letter = board[position.row][position.col];
        if dictionary.root.items.contains_key(&letter) {
            let m = board.len() as i32;
            let n = board[0].len() as i32;
            let directions = vec!['N', 'S', 'E', 'W'];
            let node = &dictionary.root.items[&letter];
            let state = State::new(position, node);
            let mut stack = Vec::new();
            stack.push(state);

            while !stack.is_empty() {
                let state = stack.pop().unwrap();
                if state.node.ends_at {
                    result_set.insert(state.node.word.clone());
                }
                for dir in &directions {
                    let mut row = state.position.row as i32;
                    let mut col = state.position.col as i32; 
                    match dir {
                        'N' => { row -= 1; }
                        'S' => { row += 1; }
                        'E' => { col += 1; }
                        'W' => { col -= 1; }
                        _ => {} // do nothing
                    }
                    let valid_row = row >= 0 && row < m;
                    let valid_col = col >= 0 && col < n;
                    if valid_row && valid_col {
                        let next = Position::new(row as usize, col as usize);
                        let letter = board[next.row][next.col];
                        if !state.seen.contains(&next) && state.node.items.contains_key(&letter) {
                            let mut seen = state.seen.clone();
                            seen.insert(next);
                            let next_state = State::with_seen(next, &state.node.items[&letter], seen);
                            stack.push(next_state);
                        }
                    }
                }
            }
        }
    }

    // TODO: Needs further optimization. real_world_1 gets TimeLimitExceeded
    pub fn find_words(board: Vec<Vec<char>>, words: Vec<String>) -> Vec<String> {
        let dictionary = Self::to_dictionary(words);
        let m = board.len();
        let n = board[0].len();

        let mut result_set = HashSet::new();
        for i in 0..m {
            for j in 0..n {
                let position = Position::new(i, j);
                Self::find_words_from(&board, &dictionary, position, &mut result_set);
            }
        }
        let mut result = Vec::with_capacity(result_set.len());
        for item in result_set {
            result.push(item);
        }
        result
    }

}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn example_1() {
        let board = vec![
            vec!['o', 'a', 'a', 'n'],
            vec!['e', 't', 'a', 'e'],
            vec!['i', 'h', 'k', 'r'],
            vec!['i', 'f', 'l', 'v']
        ];
        let words = vec![
            "oath".to_string(),
            "pea".to_string(),
            "eat".to_string(),
            "rain".to_string()
        ];
        let mut result = Solution::find_words(board, words);
        result.sort_unstable();
        assert_eq!(result, vec!["eat", "oath"]);
    }

    #[test]
    fn example_2() {
        let board = vec![
            vec!['a', 'b'],
            vec!['c', 'd']
        ];
        let words = vec!["abcd".to_string()];
        let result = Solution::find_words(board, words);
        let expected: Vec<String> = vec![];
        assert_eq!(result, expected);
    }

    #[test]
    fn real_world_1() {
        let board = vec![
            vec!['m','b','c','d','e','f','g','h','i','j','k','l'],
            vec!['n','a','a','a','a','a','a','a','a','a','a','a'],
            vec!['o','a','a','a','a','a','a','a','a','a','a','a'],
            vec!['p','a','a','a','a','a','a','a','a','a','a','a'],
            vec!['q','a','a','a','a','a','a','a','a','a','a','a'],
            vec!['r','a','a','a','a','a','a','a','a','a','a','a'],
            vec!['s','a','a','a','a','a','a','a','a','a','a','a'],
            vec!['t','a','a','a','a','a','a','a','a','a','a','a'],
            vec!['u','a','a','a','a','a','a','a','a','a','a','a'],
            vec!['v','a','a','a','a','a','a','a','a','a','a','a'],
            vec!['w','a','a','a','a','a','a','a','a','a','a','a'],
            vec!['x','y','z','a','a','a','a','a','a','a','a','a']];
        let words = vec![
            "aaaaaaaaaa","baaaaaaaaa","caaaaaaaaa","daaaaaaaaa","eaaaaaaaaa",
            "faaaaaaaaa","gaaaaaaaaa","haaaaaaaaa","iaaaaaaaaa","jaaaaaaaaa",
            "kaaaaaaaaa","laaaaaaaaa","maaaaaaaaa","naaaaaaaaa","oaaaaaaaaa",
            "paaaaaaaaa","qaaaaaaaaa","raaaaaaaaa","saaaaaaaaa","taaaaaaaaa",
            "uaaaaaaaaa","vaaaaaaaaa","waaaaaaaaa","xaaaaaaaaa","yaaaaaaaaa",
            "zaaaaaaaaa","abaaaaaaaa","bbaaaaaaaa","cbaaaaaaaa","dbaaaaaaaa",
            "ebaaaaaaaa","fbaaaaaaaa","gbaaaaaaaa","hbaaaaaaaa","ibaaaaaaaa",
            "jbaaaaaaaa","kbaaaaaaaa","lbaaaaaaaa","mbaaaaaaaa","nbaaaaaaaa",
            "obaaaaaaaa","pbaaaaaaaa","qbaaaaaaaa","rbaaaaaaaa","sbaaaaaaaa",
            "tbaaaaaaaa","ubaaaaaaaa","vbaaaaaaaa","wbaaaaaaaa","xbaaaaaaaa",
            "ybaaaaaaaa","zbaaaaaaaa","acaaaaaaaa","bcaaaaaaaa","ccaaaaaaaa",
            "dcaaaaaaaa","ecaaaaaaaa","fcaaaaaaaa","gcaaaaaaaa","hcaaaaaaaa",
            "icaaaaaaaa","jcaaaaaaaa","kcaaaaaaaa","lcaaaaaaaa","mcaaaaaaaa",
            "ncaaaaaaaa","ocaaaaaaaa","pcaaaaaaaa","qcaaaaaaaa","rcaaaaaaaa",
            "scaaaaaaaa","tcaaaaaaaa","ucaaaaaaaa","vcaaaaaaaa","wcaaaaaaaa",
            "xcaaaaaaaa","ycaaaaaaaa","zcaaaaaaaa","adaaaaaaaa","bdaaaaaaaa",
            "cdaaaaaaaa","ddaaaaaaaa","edaaaaaaaa","fdaaaaaaaa","gdaaaaaaaa",
            "hdaaaaaaaa","idaaaaaaaa","jdaaaaaaaa","kdaaaaaaaa","ldaaaaaaaa",
            "mdaaaaaaaa","ndaaaaaaaa","odaaaaaaaa","pdaaaaaaaa","qdaaaaaaaa",
            "rdaaaaaaaa","sdaaaaaaaa","tdaaaaaaaa","udaaaaaaaa","vdaaaaaaaa",
            "wdaaaaaaaa","xdaaaaaaaa","ydaaaaaaaa","zdaaaaaaaa","aeaaaaaaaa",
            "beaaaaaaaa","ceaaaaaaaa","deaaaaaaaa","eeaaaaaaaa","feaaaaaaaa",
            "geaaaaaaaa","heaaaaaaaa","ieaaaaaaaa","jeaaaaaaaa","keaaaaaaaa",
            "leaaaaaaaa","meaaaaaaaa","neaaaaaaaa","oeaaaaaaaa","peaaaaaaaa",
            "qeaaaaaaaa","reaaaaaaaa","seaaaaaaaa","teaaaaaaaa","ueaaaaaaaa",
            "veaaaaaaaa","weaaaaaaaa","xeaaaaaaaa","yeaaaaaaaa","zeaaaaaaaa",
            "afaaaaaaaa","bfaaaaaaaa","cfaaaaaaaa","dfaaaaaaaa","efaaaaaaaa",
            "ffaaaaaaaa","gfaaaaaaaa","hfaaaaaaaa","ifaaaaaaaa","jfaaaaaaaa",
            "kfaaaaaaaa","lfaaaaaaaa","mfaaaaaaaa","nfaaaaaaaa","ofaaaaaaaa",
            "pfaaaaaaaa","qfaaaaaaaa","rfaaaaaaaa","sfaaaaaaaa","tfaaaaaaaa",
            "ufaaaaaaaa","vfaaaaaaaa","wfaaaaaaaa","xfaaaaaaaa","yfaaaaaaaa",
            "zfaaaaaaaa","agaaaaaaaa","bgaaaaaaaa","cgaaaaaaaa","dgaaaaaaaa",
            "egaaaaaaaa","fgaaaaaaaa","ggaaaaaaaa","hgaaaaaaaa","igaaaaaaaa",
            "jgaaaaaaaa","kgaaaaaaaa","lgaaaaaaaa","mgaaaaaaaa","ngaaaaaaaa",
            "ogaaaaaaaa","pgaaaaaaaa","qgaaaaaaaa","rgaaaaaaaa","sgaaaaaaaa",
            "tgaaaaaaaa","ugaaaaaaaa","vgaaaaaaaa","wgaaaaaaaa","xgaaaaaaaa",
            "ygaaaaaaaa","zgaaaaaaaa","ahaaaaaaaa","bhaaaaaaaa","chaaaaaaaa",
            "dhaaaaaaaa","ehaaaaaaaa","fhaaaaaaaa","ghaaaaaaaa","hhaaaaaaaa",
            "ihaaaaaaaa","jhaaaaaaaa","khaaaaaaaa","lhaaaaaaaa","mhaaaaaaaa",
            "nhaaaaaaaa","ohaaaaaaaa","phaaaaaaaa","qhaaaaaaaa","rhaaaaaaaa",
            "shaaaaaaaa","thaaaaaaaa","uhaaaaaaaa","vhaaaaaaaa","whaaaaaaaa",
            "xhaaaaaaaa","yhaaaaaaaa","zhaaaaaaaa","aiaaaaaaaa","biaaaaaaaa",
            "ciaaaaaaaa","diaaaaaaaa","eiaaaaaaaa","fiaaaaaaaa","giaaaaaaaa",
            "hiaaaaaaaa","iiaaaaaaaa","jiaaaaaaaa","kiaaaaaaaa","liaaaaaaaa",
            "miaaaaaaaa","niaaaaaaaa","oiaaaaaaaa","piaaaaaaaa","qiaaaaaaaa",
            "riaaaaaaaa","siaaaaaaaa","tiaaaaaaaa","uiaaaaaaaa","viaaaaaaaa",
            "wiaaaaaaaa","xiaaaaaaaa","yiaaaaaaaa","ziaaaaaaaa","ajaaaaaaaa",
            "bjaaaaaaaa","cjaaaaaaaa","djaaaaaaaa","ejaaaaaaaa","fjaaaaaaaa",
            "gjaaaaaaaa","hjaaaaaaaa","ijaaaaaaaa","jjaaaaaaaa","kjaaaaaaaa",
            "ljaaaaaaaa","mjaaaaaaaa","njaaaaaaaa","ojaaaaaaaa","pjaaaaaaaa",
            "qjaaaaaaaa","rjaaaaaaaa","sjaaaaaaaa","tjaaaaaaaa","ujaaaaaaaa",
            "vjaaaaaaaa","wjaaaaaaaa","xjaaaaaaaa","yjaaaaaaaa","zjaaaaaaaa",
            "akaaaaaaaa","bkaaaaaaaa","ckaaaaaaaa","dkaaaaaaaa","ekaaaaaaaa",
            "fkaaaaaaaa","gkaaaaaaaa","hkaaaaaaaa","ikaaaaaaaa","jkaaaaaaaa",
            "kkaaaaaaaa","lkaaaaaaaa","mkaaaaaaaa","nkaaaaaaaa","okaaaaaaaa",
            "pkaaaaaaaa","qkaaaaaaaa","rkaaaaaaaa","skaaaaaaaa","tkaaaaaaaa",
            "ukaaaaaaaa","vkaaaaaaaa","wkaaaaaaaa","xkaaaaaaaa","ykaaaaaaaa",
            "zkaaaaaaaa","alaaaaaaaa","blaaaaaaaa","claaaaaaaa","dlaaaaaaaa",
            "elaaaaaaaa","flaaaaaaaa","glaaaaaaaa","hlaaaaaaaa","ilaaaaaaaa",
            "jlaaaaaaaa","klaaaaaaaa","llaaaaaaaa","mlaaaaaaaa","nlaaaaaaaa",
            "olaaaaaaaa","plaaaaaaaa","qlaaaaaaaa","rlaaaaaaaa","slaaaaaaaa",
            "tlaaaaaaaa","ulaaaaaaaa","vlaaaaaaaa","wlaaaaaaaa","xlaaaaaaaa",
            "ylaaaaaaaa","zlaaaaaaaa","amaaaaaaaa","bmaaaaaaaa","cmaaaaaaaa",
            "dmaaaaaaaa","emaaaaaaaa","fmaaaaaaaa","gmaaaaaaaa","hmaaaaaaaa",
            "imaaaaaaaa","jmaaaaaaaa","kmaaaaaaaa","lmaaaaaaaa","mmaaaaaaaa",
            "nmaaaaaaaa","omaaaaaaaa","pmaaaaaaaa","qmaaaaaaaa","rmaaaaaaaa",
            "smaaaaaaaa","tmaaaaaaaa","umaaaaaaaa","vmaaaaaaaa","wmaaaaaaaa",
            "xmaaaaaaaa","ymaaaaaaaa","zmaaaaaaaa","anaaaaaaaa","bnaaaaaaaa",
            "cnaaaaaaaa","dnaaaaaaaa","enaaaaaaaa","fnaaaaaaaa","gnaaaaaaaa",
            "hnaaaaaaaa","inaaaaaaaa","jnaaaaaaaa","knaaaaaaaa","lnaaaaaaaa",
            "mnaaaaaaaa","nnaaaaaaaa","onaaaaaaaa","pnaaaaaaaa","qnaaaaaaaa",
            "rnaaaaaaaa","snaaaaaaaa","tnaaaaaaaa","unaaaaaaaa","vnaaaaaaaa",
            "wnaaaaaaaa","xnaaaaaaaa","ynaaaaaaaa","znaaaaaaaa","aoaaaaaaaa",
            "boaaaaaaaa","coaaaaaaaa","doaaaaaaaa","eoaaaaaaaa","foaaaaaaaa",
            "goaaaaaaaa","hoaaaaaaaa","ioaaaaaaaa","joaaaaaaaa","koaaaaaaaa",
            "loaaaaaaaa","moaaaaaaaa","noaaaaaaaa","ooaaaaaaaa","poaaaaaaaa",
            "qoaaaaaaaa","roaaaaaaaa","soaaaaaaaa","toaaaaaaaa","uoaaaaaaaa",
            "voaaaaaaaa","woaaaaaaaa","xoaaaaaaaa","yoaaaaaaaa","zoaaaaaaaa",
            "apaaaaaaaa","bpaaaaaaaa","cpaaaaaaaa","dpaaaaaaaa","epaaaaaaaa",
            "fpaaaaaaaa","gpaaaaaaaa","hpaaaaaaaa","ipaaaaaaaa","jpaaaaaaaa",
            "kpaaaaaaaa","lpaaaaaaaa","mpaaaaaaaa","npaaaaaaaa","opaaaaaaaa",
            "ppaaaaaaaa","qpaaaaaaaa","rpaaaaaaaa","spaaaaaaaa","tpaaaaaaaa",
            "upaaaaaaaa","vpaaaaaaaa","wpaaaaaaaa","xpaaaaaaaa","ypaaaaaaaa",
            "zpaaaaaaaa","aqaaaaaaaa","bqaaaaaaaa","cqaaaaaaaa","dqaaaaaaaa",
            "eqaaaaaaaa","fqaaaaaaaa","gqaaaaaaaa","hqaaaaaaaa","iqaaaaaaaa",
            "jqaaaaaaaa","kqaaaaaaaa","lqaaaaaaaa","mqaaaaaaaa","nqaaaaaaaa",
            "oqaaaaaaaa","pqaaaaaaaa","qqaaaaaaaa","rqaaaaaaaa","sqaaaaaaaa",
            "tqaaaaaaaa","uqaaaaaaaa","vqaaaaaaaa","wqaaaaaaaa","xqaaaaaaaa",
            "yqaaaaaaaa","zqaaaaaaaa","araaaaaaaa","braaaaaaaa","craaaaaaaa",
            "draaaaaaaa","eraaaaaaaa","fraaaaaaaa","graaaaaaaa","hraaaaaaaa",
            "iraaaaaaaa","jraaaaaaaa","kraaaaaaaa","lraaaaaaaa","mraaaaaaaa",
            "nraaaaaaaa","oraaaaaaaa","praaaaaaaa","qraaaaaaaa","rraaaaaaaa",
            "sraaaaaaaa","traaaaaaaa","uraaaaaaaa","vraaaaaaaa","wraaaaaaaa",
            "xraaaaaaaa","yraaaaaaaa","zraaaaaaaa","asaaaaaaaa","bsaaaaaaaa",
            "csaaaaaaaa","dsaaaaaaaa","esaaaaaaaa","fsaaaaaaaa","gsaaaaaaaa",
            "hsaaaaaaaa","isaaaaaaaa","jsaaaaaaaa","ksaaaaaaaa","lsaaaaaaaa",
            "msaaaaaaaa","nsaaaaaaaa","osaaaaaaaa","psaaaaaaaa","qsaaaaaaaa",
            "rsaaaaaaaa","ssaaaaaaaa","tsaaaaaaaa","usaaaaaaaa","vsaaaaaaaa",
            "wsaaaaaaaa","xsaaaaaaaa","ysaaaaaaaa","zsaaaaaaaa","ataaaaaaaa",
            "btaaaaaaaa","ctaaaaaaaa","dtaaaaaaaa","etaaaaaaaa","ftaaaaaaaa",
            "gtaaaaaaaa","htaaaaaaaa","itaaaaaaaa","jtaaaaaaaa","ktaaaaaaaa",
            "ltaaaaaaaa","mtaaaaaaaa","ntaaaaaaaa","otaaaaaaaa","ptaaaaaaaa",
            "qtaaaaaaaa","rtaaaaaaaa","staaaaaaaa","ttaaaaaaaa","utaaaaaaaa",
            "vtaaaaaaaa","wtaaaaaaaa","xtaaaaaaaa","ytaaaaaaaa","ztaaaaaaaa",
            "auaaaaaaaa","buaaaaaaaa","cuaaaaaaaa","duaaaaaaaa","euaaaaaaaa",
            "fuaaaaaaaa","guaaaaaaaa","huaaaaaaaa","iuaaaaaaaa","juaaaaaaaa",
            "kuaaaaaaaa","luaaaaaaaa","muaaaaaaaa","nuaaaaaaaa","ouaaaaaaaa",
            "puaaaaaaaa","quaaaaaaaa","ruaaaaaaaa","suaaaaaaaa","tuaaaaaaaa",
            "uuaaaaaaaa","vuaaaaaaaa","wuaaaaaaaa","xuaaaaaaaa","yuaaaaaaaa",
            "zuaaaaaaaa","avaaaaaaaa","bvaaaaaaaa","cvaaaaaaaa","dvaaaaaaaa",
            "evaaaaaaaa","fvaaaaaaaa","gvaaaaaaaa","hvaaaaaaaa","ivaaaaaaaa",
            "jvaaaaaaaa","kvaaaaaaaa","lvaaaaaaaa","mvaaaaaaaa","nvaaaaaaaa",
            "ovaaaaaaaa","pvaaaaaaaa","qvaaaaaaaa","rvaaaaaaaa","svaaaaaaaa",
            "tvaaaaaaaa","uvaaaaaaaa","vvaaaaaaaa","wvaaaaaaaa","xvaaaaaaaa",
            "yvaaaaaaaa","zvaaaaaaaa","awaaaaaaaa","bwaaaaaaaa","cwaaaaaaaa",
            "dwaaaaaaaa","ewaaaaaaaa","fwaaaaaaaa","gwaaaaaaaa","hwaaaaaaaa",
            "iwaaaaaaaa","jwaaaaaaaa","kwaaaaaaaa","lwaaaaaaaa","mwaaaaaaaa",
            "nwaaaaaaaa","owaaaaaaaa","pwaaaaaaaa","qwaaaaaaaa","rwaaaaaaaa",
            "swaaaaaaaa","twaaaaaaaa","uwaaaaaaaa","vwaaaaaaaa","wwaaaaaaaa",
            "xwaaaaaaaa","ywaaaaaaaa","zwaaaaaaaa","axaaaaaaaa","bxaaaaaaaa",
            "cxaaaaaaaa","dxaaaaaaaa","exaaaaaaaa","fxaaaaaaaa","gxaaaaaaaa",
            "hxaaaaaaaa","ixaaaaaaaa","jxaaaaaaaa","kxaaaaaaaa","lxaaaaaaaa",
            "mxaaaaaaaa","nxaaaaaaaa","oxaaaaaaaa","pxaaaaaaaa","qxaaaaaaaa",
            "rxaaaaaaaa","sxaaaaaaaa","txaaaaaaaa","uxaaaaaaaa","vxaaaaaaaa",
            "wxaaaaaaaa","xxaaaaaaaa","yxaaaaaaaa","zxaaaaaaaa","ayaaaaaaaa",
            "byaaaaaaaa","cyaaaaaaaa","dyaaaaaaaa","eyaaaaaaaa","fyaaaaaaaa",
            "gyaaaaaaaa","hyaaaaaaaa","iyaaaaaaaa","jyaaaaaaaa","kyaaaaaaaa",
            "lyaaaaaaaa","myaaaaaaaa","nyaaaaaaaa","oyaaaaaaaa","pyaaaaaaaa",
            "qyaaaaaaaa","ryaaaaaaaa","syaaaaaaaa","tyaaaaaaaa","uyaaaaaaaa",
            "vyaaaaaaaa","wyaaaaaaaa","xyaaaaaaaa","yyaaaaaaaa","zyaaaaaaaa",
            "azaaaaaaaa","bzaaaaaaaa","czaaaaaaaa","dzaaaaaaaa","ezaaaaaaaa",
            "fzaaaaaaaa","gzaaaaaaaa","hzaaaaaaaa","izaaaaaaaa","jzaaaaaaaa",
            "kzaaaaaaaa","lzaaaaaaaa","mzaaaaaaaa","nzaaaaaaaa","ozaaaaaaaa",
            "pzaaaaaaaa","qzaaaaaaaa","rzaaaaaaaa","szaaaaaaaa","tzaaaaaaaa",
            "uzaaaaaaaa","vzaaaaaaaa","wzaaaaaaaa","xzaaaaaaaa","yzaaaaaaaa",
            "zzaaaaaaaa"];
        let words = words.into_iter().map(|s| s.to_string()).collect();
        let mut result = Solution::find_words(board, words);
        result.sort_unstable();
        let expected = vec![
            "kjaaaaaaaa","paaaaaaaaa","jkaaaaaaaa","ihaaaaaaaa","tuaaaaaaaa",
            "deaaaaaaaa","eaaaaaaaaa","raaaaaaaaa","poaaaaaaaa","hiaaaaaaaa",
            "waaaaaaaaa","opaaaaaaaa","uaaaaaaaaa","yaaaaaaaaa","cbaaaaaaaa",
            "taaaaaaaaa","haaaaaaaaa","uvaaaaaaaa","mbaaaaaaaa","klaaaaaaaa",
            "vwaaaaaaaa","ijaaaaaaaa","ghaaaaaaaa","dcaaaaaaaa","jiaaaaaaaa",
            "vuaaaaaaaa","xwaaaaaaaa","bcaaaaaaaa","saaaaaaaaa","mnaaaaaaaa",
            "azaaaaaaaa","rsaaaaaaaa","naaaaaaaaa","fgaaaaaaaa","qraaaaaaaa",
            "noaaaaaaaa","lkaaaaaaaa","laaaaaaaaa","gfaaaaaaaa","zyaaaaaaaa",
            "sraaaaaaaa","daaaaaaaaa","jaaaaaaaaa","caaaaaaaaa","staaaaaaaa",
            "yzaaaaaaaa","iaaaaaaaaa","edaaaaaaaa","cdaaaaaaaa","utaaaaaaaa",
            "feaaaaaaaa","hgaaaaaaaa","rqaaaaaaaa","vaaaaaaaaa","qpaaaaaaaa",
            "aaaaaaaaaa","onaaaaaaaa","pqaaaaaaaa","wvaaaaaaaa","qaaaaaaaaa",
            "zaaaaaaaaa","faaaaaaaaa","baaaaaaaaa","efaaaaaaaa","tsaaaaaaaa",
            "gaaaaaaaaa","xyaaaaaaaa","oaaaaaaaaa","kaaaaaaaaa"
        ];
        let mut expected: Vec<String> = expected.into_iter().map(|s| s.to_string()).collect();
        expected.sort_unstable();
        assert_eq!(result, expected);
    }

}
