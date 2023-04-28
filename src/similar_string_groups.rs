use std::collections::HashMap;

/// Two strings `X` and `Y` are similar if we can swap two letters (in
/// different positions) of `X`, so that it equals `Y`. Also two strings are
/// similar if they are equal.
///
/// For example, `"tars"` and `"rats"` are similar (swapping at positions `0`
/// and `2`), and `"rats"` and `"arts"` are similar, but `"star"` is not
/// similar to `"tars"`, `"rats"`, or `"arts"`.
///
/// Together, these form two connected groups by similarity:
/// `{"tars", "rats", "arts"}` and `{"star"}`. Notice that `"tars"` and
/// `"arts"` are in the same group even though they are not similar. Formally,
/// each group is such that a word is in the group if and only if it is similar
/// to at least one other word in the group.
///
/// We are given a list `strs` of strings where every string in `strs` is an
/// anagram of every other string in `strs`. How many groups are there?
struct Solution;

impl Solution {

    fn is_connected(a: &str, b: &str) -> bool {
        let a = a.as_bytes();
        let b = b.as_bytes();

        let mut diff = 0;
        let mut result = true;

        for i in 0..a.len() {
            if a[i] != b[i] {
                diff += 1;
                if diff > 2 {
                    result = false;
                    break;
                }
            }
        }

        result
    }

    pub fn num_similar_groups(strs: Vec<String>) -> i32 {
        let mut words: HashMap<usize, Vec<&String>> = HashMap::new();
        let mut groups: HashMap<&String, usize> = HashMap::new();

        for i in 0..strs.len() {
            let word = &strs[i];
            if !groups.contains_key(word) {
                let mut group = Vec::new();
                group.push(word);
                words.insert(i, group);
                groups.insert(word, i);
            }
        }

        for i in 0..strs.len()-1 {
            for j in (i+1)..strs.len() {
                let word_i = &strs[i];
                let word_j = &strs[j];
                let group_i = groups[word_i];
                let group_j = groups[word_j];

                if group_i != group_j && Self::is_connected(word_i, word_j) {
                    let mut words_j = words.remove(&group_j).unwrap();

                    for word in &words_j {
                        groups.entry(word).and_modify(|g| *g = group_i);
                    }
                    words.entry(group_i).and_modify(|w| w.append(&mut words_j));
                }
            }
        }

        words.len() as i32
    }

}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn example_1() {
        let orig = vec!["tars", "rats", "arts", "star"];
        let strs: Vec<String> = orig.iter().map(|s| s.to_string()).collect();
        let result = Solution::num_similar_groups(strs);
        assert_eq!(result, 2);
    }

    #[test]
    fn example_2() {
        let orig = vec!["omv", "ovm"];
        let strs = orig.iter().map(|s| s.to_string()).collect();
        let result = Solution::num_similar_groups(strs);
        assert_eq!(result, 1);
    }

    #[test]
    fn example_3() {
        let orig = vec!["jqmknamokpmtavoxvfwxvzgwrmohjuyldjfsjbftnoirbgajjciurynqivkniqyenanhkdahsjkxsiowyxenibnaodavgvkvbcexakbjuljbvmdlaavqlpihvpwaipekvuxoeqvueyqtlbftzkyzyugkzygotbkdmlgkwjwvjwjhhqzmdaqhgfxekogkvaqsdeifubvbplklsludrntmemrygxqhpvvcjeigbmwsapivrksllvdcyzuviu","bmqlkudnzeviaeqvquzmbappzksbvauwxuqfpwkfvvkbeidgcmqleqevnylhxubyvjdhqkayngerithocfvjkkyudcwllguvwjhbvjopnkvxhljddjgfjryknyuiomtkmhxritjadkeixazaiuqgyjsyvyaiisfvqigfilorpkpmvlvamxvjmsvmatobhxhwjnbblzjosxsatqkpwwvalyodkrkagznbbnslvjnevodcugeggktiaemwro","riellsqyeiuqzjpapwvmpqyezartjhflvjwuqkvbfjuydbitkwypnnbksjbtjbaqiuazlewvdwbxevlndxpilqgvmoyavavgaomyiokxuajyhagixcjdeisrvvakfsjknqehkhvkvmnmwjgmevqhrnkpawlstkxfvoyudaznivhgkbbdpditucrhtqehbxiagraxuikldnzlukvxfnkkjgmlksmsgoqvdmebvjgyclboomvwvfyogojucz","zvqrvglckdqebxpuclgbizragbnwrwnabpqxuxadkuwdgxecmsvejibnjheqkraxssuqoywromyhhzvklkdjxiaeihimkkofdungkmskfjujkymvvvkdaygcnlymjyvvkxywzanqikplkwmephuafavyfveieobqbhjnaoyjogxgntbqsddiftihqteieomfjvgylvvoqvbjptupkzwtjnlijslbgkmiuvhlaavtvdljbpwsoulvazvarm","olwrmjdvzgnsinneshaqfwqbmqezmmbvvykbdqqegntsxoyyktwbxnbqvbhaubkmgtkjqjugkaelribjeaxaciepkrbfdhjqmvnlevykdsnrvujluvaxyykuphoyoakmaxwobzpivilvtoifealvanjovtgkjlqjswrjpvfvhkvilkrtmwikigpmdkiogzqygycigaxvdvzxxpadudnudylujjhmhfkclcavavpvswjlubeioukwfhgesz","ivejavaxvbmvplklwgyawesimhovskbhivsinnobrkhddetfamkhkdkoivxcajsgtxezatcfevjlbqwbkxidqyyyknkykujkiwvaovidjzmoouqnwrmpibrpmmwgnkbvnsjjfymdgyqlalnfvyicvboungbjmuuhfghtjbsuaqlzateyvregldevrkxlkuvboyepkvoxxcamgdfsjqtpguuqgaewlazvqrvuzzqjjhqilipwpxkhjvland","pohyyliinotagjkbbdrdvxgkhqmvbnsjuivtjrzzjdxenbvlndnakptjzfoixifmujdbavnuwvxsgmefkjqsmcgqwpidoilulcizeojklwbsuhpaotkiileujkkrvbblyzecwyjedphjokahaaeahwqqalkqmfmfvvuvqlxtbmrvahyyidibvkvfsqxptvezxyxgnoqwahmdskuajvkogklucagggevkqanjvmvvmrsgkrvupynwweblyy","ieonegohvqdijnjwziloffltzumageoynbiyureqtezkblpbdvmvbacijbwfenzdgoyvqvsyckhaoksmbkkrjmjjynskmglfvoskbwtxtaiwlkdwyabmjvhjphkuayreqhakvghmztcupximpxprqbejgxbjuhfiudwjvlsilpaegmtqvsrqanuvkvdlasvbknhrwlakxiqcvvlaajfzxvkdiagnvkdqgdyqkyuuvwupoxmiejlvgxvyon","gabjzebcibeprlntvmlvhdowqsutwfcobbjyvgktwqdhbukauqlhvvxvkvfblkvckiwvkunjjiogwupmfstkrasnpgjeqawvlyzyokkveqlkqvdkjegajyzhvybrmzoippatpwfmvveoxijighkrnlgqvadjiryqmxonepkuvukgauarbwlvoamlxixinjytmbaokylauvvlgzmqhfbiycyfuedjnsqjmhsazaskenmeidgidsxdhjxxnd","wdjbpfcewhuvjfqdyqeomfptuqxdtjiwwryjalbitkijkhkajglxgawkvesuydghfhxzirzgydyabooitxobgldmsfchxinipvaessorcxayxbdvvhnouqtkvbuyneywdzldhvkqsuwingpugklemgmvacrkekvpvnaulablvaumaabjvrevjovvrjtlvmifxnyneapwupvjnkmibzqvhkqbqykzavqlgkbqvljizesokmsjlmmoknjigk","gkgvvevavbufeqpyllaomzscsnirdrekaxwsttyaqkerxkwajvkfyvncomhzlukenknkwcbhabhpjgvramogpiauglkxbaithsyuijrijlbkdeopjyxwiufoqupldkkzxwvqktqsqjogbbngqvmphexqyigvdnmxdkdmzujyveuksziavmnhnvtovrmdavckqbwtgbvaiflabuvdjjbqvzlidjlfynejlushwamiyvvjjmlyefgxhiwpoo","kgsvaulqzodwgbgtjxhiaqgqskbbohditezlgiixzumrjdvwkedwvpazifmgxvkpbuevpbvfveykqngjrasvxrbiqbhqclevvnvnfqqtkggiwrirqtnonkknkcylvaqvjulkmsmhfslbwdvjpwmgakneuotdyjovmbvaszypvlajajjbcaauyltaewdikvnomyixmviyaapxvcmldfbjyekzjjiwkuusefuxooyhnkhkmydrxhluoljehp","zwrhbdinbkaikqotbsbipdqkfmeyfjsgqbhkirvwlcvrfsuomxkekxlyjrovknyveaqxhhlxvppvsykbnpiitdwjklvteomcziqwaaelavalbpvjugdrvdovkmgwhkwqveafapojbyvjrnmsekwgukmiabtazvhqexoukqhjktojhodgclljeeuvgysalzivzyguftuxnsxayfivwzjmyaajbinlvcnxdbjdnqipyknmuvvmuujlgggmdq","xhgcsxeuqvqhmegcjbqdsjvrpzvuokyzvjnqdfaajgtafbvwpebvkhmgyixolkwroodtnvmkcvtnuvejkyqtgyyjardvrpbdvasiowkjkkulomldlxnuayewupgpwozqxvlfpnanexbuiqmtjfbjkwohmxdduoxmknvvseinlkkiyyrfiluiazrajqvhvkswidspvmqbblstelabivjmemwakuahkvjzgyeialvhlqgbgjzyaihbgicfnk","akyzdjdzztxvuduxkqklievwnfbozdxaojbdklpefiaawobamqezvqjpeuskgmrrlyaquftcgribbmkpdviamegektoblvevjrvvtnukmvwpgmvlakexynnsbahgvwobsejivgblljhikxduxrvacqqdkpighfhkyugkkysjlgmtmsljjosihvynmcmlcaqsnhvfjbxxeqtprquvywnydkpqvkagavwjnwyfnhaziijywohjuuovlviiob","aplgbwviabkgbnqnwrxgsqwevyhaeddxdvijjtqiutkvdvnbsplsjsouipsjvhcbydopgkkonmadgmmbynvlxekownixavuyybkvufkkkfibbzvvmkbcrhkjilkergkmaywqiuhrewamzjmvzyhholhajlljvzxkedvqyeafxlhglouqpdmazikrqusvgffmzavcntyjckvvqiejgqajtwdueaixwtjqfnjenvpbrgvuaospltmuiyxool","ikaiaihivqfvwqixouhzwkaidlvksgyhdnrlzyvbqzabwuvjelpnakqnevkuaalejyhbymsxlsnpqxynvvlhhikqctrswduvvvfqjmmyhoaoxwpiaqdueklgsdosnbvkbmjdmcawfregiuikbvbfjokjylkugqjxesgjigvpvwouggajtlobrmiennmxkpdoreftkokvxhjbyemlakwjgmgavdvrtbztcdbyvvzujxtzcepnfjmyalkupq","ueevkksbxmxvdutenusguauvloqdzonkrkqyvrlaithgeluavkhpxqaaaoysrpezajvkmpdatuvqyksmhvapkjnmgxvuyuxjlbwkrbfvkwlqrtepbbjzyovfkqveifajygqcinbfgdeasmzjaxibliqogbvwndhfgiakygabxdhovmghqskmyvzpuivwqknmwlihnkzojpyjsjrjvkclmwinwgjljccljoeilndtbovhdxydmwbvtvifei","zausjriawclxmheuujynfiabieqrvqdpibykxztgdxkboakhhimwwhvhqjipbwuglnlugrigojkynrvpffekiblduyzvanrhjgajskejsqeomuyxbgjynssnbmklejbpvkktvanqoxcuvfyeakhkdodgkqpieoitavwmavrgqxvtdjmhwpcgyvoqisbuvykebllunvvjxlyfclaqafavdtldzqewvvvoktsklnjdzmmxvabmvomzigpkjw","egmbkbxacjymangbysmukgimajokpnqqdhiolyajahnzpkjeikoumvjgmafazxdkqcwzvlrdzuqvslwurtbbslecumwnmguxngvobinkkhhmwgpxawevlveejrydnivvbgkfhblijcatjjwjwadupxqsiveqtsqofpldeykjiywlarfdypiktprvqkuvuqvbvsfodqknvvlkijojzbioxheevxadzhvvamtotnaglyvyxgisublvfkhkry","ieonegohvqdijnjwziloffwtzumageoynbiyureqtezkblpbdvmvbacijblfenzdgoyvqvsyckhaoksmbkkrjmjjynskmglfvoskbwtxbaiwlkdwyatmjvhjphkuayreqhakvghmztcupximpxprqbejwxbjuhfiudwjvlsilpaegmtqvsrqanuvkvdlasvbknhrglakxiqcvvlaajfzxvkdiagnvkdqgdyqkyuuvwupoxmiejlvgxvyon","svarvvkgudabhnjazoiffpmqwdekdwftwkcisvkbkyusjuqglbnyuslyauqiwqmyoniexvoqivlvkbpeivvfgrwhvvajnyakskktzvdlroaevjbdxvluaianwmkdvljoebpygkjzelasobmgptyjdanvqluuqgmpxhelucxxyefvbgcktvaiztnhyoibkoqwqxdnzjjogejvwxymhmlhiurcriljnxjskjkrvztqempmpbgkfhbaigdahm","kirckfiiujesbjkqbjthvamlmylartnaumuxjwaaqlwnrqqdvmanvbpgejxvdxyykofrhlngzkcwbquiiucovkjioboideggkqsnmkkdaggsavibwzfdmoluktpybvhgfxehuojbmjcivehnwvlsiekjeaukdvzwaeyyvpdmrjfxqawnajyoeqgiddsytvwyzpangvklvbjtkhvusovyohrvkbkhvjitvxxqpgslmlbmellpquzpnafxvz","hadqkesxllraekarnegwjqoduaawrpokvusqcauylyvrysnlvlkqxwkxvpvpieuvvnjvjgvjtcuqdxivozitieqdjdnexhbeaclvphlaqmwmsxxjljfbfiogkkhkjtzxyvakmnofgmhuvsyabbmpjdesmbbhkzhjbfkndmyjbkkqavjvdbfgluvivgiealimwpzqsfgoivzyjunyowievipvcuytdnlrwqoatzikbbahwrmmgngkuytgok","mvmbmhdqzrvxdubiajqgohcggpjmkhytvfjakjokfdvimlueplyyldarrfdinvhwjalafgcmwiwhgzxnnzaliivvgvbeqvjetuvdfasaeewklqhyivebzetgjohmotvmvbvbgvofiiezumkjdkxjknrcjopsqxrypztibqikbyrsokvoykpmyugodqkwleaxyvugakkbqjcxnwvhpxxavwdapsuinjetljwlanyqlnuaubkslvkqskubns","olwrmjdvzgnsinneshaqfwqbmqezmmbvvykbdqqegntsxoyyktwbxnbqvbhaubkmgtkjqjugkaelribjeaxaciepkrbfdhjqmvnlevykdsnrvujluvaxyykuphoyoakmaxwobzpivilvtoifealvanjovtgkjlqjswrjpvfvhkvilkrtmwikigpmdkiogzvygycigaxvdvzxxpadudnudylujjhmhfkclcaqavpvswjlubeioukwfhgesz","xhgcsxeuqvqhdegcjbqdsjvrpzvuokyzvjnqdfaajgtafbvwpebvkhmgyixolkwroodtnvmkcvtnuvejkyqtgyyjardvrpbdvasiowkjkkulomlmlxnuayewupgpwozqxvlfpnanexbuiqmtjfbjkwohmxdduoxmknvvseinlkkiyyrfiluiazrajqvhvkswidspvmqbblstelabivjmemwakuahkvjzgyeialvhlqgbgjzyaihbgicfnk","wqijnqulcanqvuewodakbvsfxwikxoxanltjdiogybkcrvhojaqybfhlkgfejyjvzqwpivledsstlgjplkkawtavpncjvbmetvxkdgwkmbmblvhgusvlxrkboymmvjpkuwburumjainunnrprrlcvtgykqsquhdaqzekkngjgoxbvekihmiyyivzkvueehygabfvpwosikiidzjfvjxlldvjxhpqteiymhugdnamqzdveomaazsfbovaya","kxbggdwqihopvldajvbmmznolsdvujoovivzmnjfyuejjdszlyhoplaainckjkyznajnznqkdvvauuyswykypfaetvrgjvsdzxiruibplgidkfawyvkfeubwhvqrgkkvfmtvjtwememqhmgaiubsxpfegakbtuxpxrojjmgrqtldmqlbkchklixispsvanhuqokbkbbgbmvqqhcywnveaytgeuyalvvvnxqoiikevlkclawewohadjrijx","lyykqqzwqvgmajbzobfmrtfprnarnlveeljibbxgqwhdugknjdpqvnpkeekvdiygxppitdgvxvjqyejgkjihzdoiubsqhatlqyeakladufcwzbyzycvpcnmbuiujlzvfadhybljwiulnaamkheoojegshbdivkvasnyudukogamxbisiwvinoavwkvwyxvshirekjupkxfmvjmjsggxrkrlvvmovqbkkkmttjaaoltcnohvwfvmelquasx","pinndzmotucqyalkewodvujphdyqybekvkafnkiueovlmmuevqbhxgvmxvwkgxcougqwgjipaqlefsayamrmbabkkjhikpixvsstggiwktfiiiixotssvzwzjsvbsgakdawhlzyjmgjnuyjbvpnuxatrloupoeyerlnbjkjmwrqakzndjekmavhkgxcyybvvldrepcfaovjvqkdbjvqquhyflmzdaovifajlwnvtbhgkhivnlxlubqervd","dzdozvwjsaijelefqsgyapusvmpnojjbzojtienlkaqbefjeuyxkvruymgvnllmkigqafwirwkmfnndikkrvpkjebcibuvznwnmhzoiykoggauvmejiqabackahyxkafkvaegbwzvqklurmxagddjwkpwqvibhqtxxvuvshgltsddsyvxyvotbdalqscuxvitbfrbovnumhjkhiwthyaycvvqiybkupllrmgedojmxlpjqhvnokapegjvl","fpykyddtjjibbakvqjdfvvknyitauoqyckkhcanehqgianrickgywikeyoabkamozpwqfnbmivnwpevbqvjuudezptxpushgglmpskvvemukkvmgzxgdfklnymrivjemlsgmydlxabnjhslqgmtuuakungzvgvxwajqyixqhiljmilkwbulbxhqpiywrofdjewadsjkbrvszsqaobzlnfxtohrhxvbukoleijvreatjcevjowalvavdovv","ueevkksbxmxvdutenusguauvloqdzonkrkqyvrlaithgeluavkhpxqaaaoysrpezajvkmpdatuvqykswhvapkjnmgxvuyuxjlbwkrbfvkwlqrtepbbjzyovfkqveifajygqcinbfgdeasmzjaxibliqogbvmndhfgiakygabxdhovmghqskmyvzpuivwqknmwlihnkzojpyjsjrjvkclmwinwgjljccljoeilndtbovhdxydmwbvtvifei","uqijsmtvriacoqfdlotnpbrokmkehnleqqkgeuvbvmjwpvlglvtgdkwsbiivlxxmjcraebdjswdquinzyvfkogvhfhujwmfsdlyidunpyrvqaqlvilknzvkectskiwieijpxbyfoedyaugyvmwzeqvuwakjgnbepvejnowinhzukajmaaxymgvhzovdabqgamltxtkzhklbggapvfmybjqyuhbhsobjxolxukaikxvasvvayjkpkjcnrrd","wxvyqeidovwhvfscbgvylakunmtjsbnanepehfkkkvqmajaskmidphmzxpkbkgjxvtalkdsaupybtamcvrwkmrvqbkeojllrtpsigdhaqluueozjvsaajnayljqgzvbajlbknbykdzixegbnetivcrvbwaovejtfycddoynywvfouwlqkirkqeivahugyqvpziloxvgijbuudlnqymdulkkzmhohpjwrhvfnxvwfimqiuxgxigvjgoesmj","riellsqyeiuqzjpapwvmpqyezartjhflvjwuqkvbfjuydbitkwypnnbksjbtjbaqiuazlewvdwbxevlndxpmlqgvmoyavavgaomyiokxuajyhagixcjdeisrvvakfsjknqehkhvkvmniwjgmevqhrnkpawlstkxfvoyudaznivhgkbbdpditucrhtqehbxiagraxuikldnzlukvxfnkkjgmlksmsgoqvdmebvjgyclboomvwvfyogojucz","ojwykmncocsubpameljxceikiqoyiulqluvjxyfztkrxnmeljvonndwalggaivhbbkotliuvnbvabpvdbokesqvvgiwojyhajtniajbkzvurmskzcplyfpiayfadklqdndbgkvmhgvkupuktvmguhkljxukfojwrvqagpvzygavrlhsuyeedtzmvkqejtavbjhlazyfwewfxysqxdqimrvmnbpvhwivqseqrbxmigdskkngihwadajxeoj","ivejavaxvbmvplklogyawesimhovskbhivsinnobrkhddetfamkhkdkoivxcajsgtxezatcfevjlbqwbkxidqyyyknkykujkiwvawvidjzmoouqnwrmpibrpmmwgnkbvnsjjfymdgyqlalnfvyicvboungbjmuuhfghtjbsuaqlzateyvregldevrkxlkuvboyepkvoxxcamgdfsjqtpguuqgaewlazvqrvuzzqjjhqilipwpxkhjvland","hadqkesxllraekarnegwjqoduaawrpowvusqcauylyvrysnlvlkqxwkxvpvpieuvvnjvjgvjtcuqdxivozitieqdjdnexhbeaclvphlaqmkmsxxjljfbfiogkkhkjtzxyvakmnofgmhuvsyabbmpjdesmbbhkzhjbfkndmyjbkkqavjvdbfgluvivgiealimwpzqsfgoivzyjunyowievipvcuytdnlrwqoatzikbbahwrmmgngkuytgok","kirckfiiujesbjkqbjthvamlvylartnaumuxjwaaqlwnrqqdvmanvbpgejxvdxyykofrhlngzkcwbquiiucovkjioboideggkqsnmkkdaggsavibwzfdmoluktpybvhgfxehuojbmjcivehnwvlsiekjeaukdvzwaeyyvpdmrjfxqawnajyoeqgiddsytvwyzpangvklvbjtkhmusovyohrvkbkhvjitvxxqpgslmlbmellpquzpnafxvz","ouijmmjiteeauugfxovoguwsdlibsrjtgbkbxehkiuahrirwvjsmwwvebypvmdmalkyvkkdynvqibbwxpkclyozeeegevatukmkdbqxewlyhvinolrhbalgsapvynftlqqpxnlfkclnvhhcpxgxknajbifyqzfjukizbosntgjjuohsqhkjyujcjoavvdqvwtnglaoivamvkgasybwmupkkrkjzvjmzgfidazyqvnrxdmqaplevdqvviad","dbtxpbqtbhkjgazvfiqvbmflasopmjibejdiuayizylxvrvxykqawrygjtlvshyaiwlbnuajzpzblrhigxedbmvlmveftggvhnfusveitvvvojelvznwfmuirkkbqmwakubekcxjqnfpknnoavauskueyavphcsyowqxlccdaeykptkiqdiueawvokolusqgkkvxkjbvoqdjmghnmdmjaggjwzjoykiqwravgdnmlokndjvyhiselhurxp","dbtxpbqtbhkjgazvfiqvbmflasopmjibewdiuayizylxvrvxykqawrygjtlvshyaiwlbnuajzpzblrhigxedbmvlmveftggvhnfusveitvvvojelvznwfmuirkkbqmjakubekcxjqnfpknnoavauskueyavphcsyowqxlccdaeykptkiqdiueawvokolusqgkkvxkjbvoqdjmghnmdmjaggjwzjoykiqwravgdnmlokndjvyhiselhurxp","ogjiviqmvgagejrvvzkokjoudnxilycihrsxvvvyeaykghpkmwtttjrsiwnekykjjbkmidvqwjfpvaouvoowoxaqywdivghqbkqyhgflvfhlpsbmvmjuhcczflqbgmneloyvnjzdxlzpnplxseticqlbxiuevkxnuikadzjgmaenzkuvjhuagqvmblybifybkawjwuqpaupshtdmeqdlbvfkektrvuagsrdnajlebbaaivnsykomxkrwad","akyzdjdzztxvuduxkqklievwnfbozdxaojbdklpefiaawobamqezvqjpeuskgmrrlyaquftcgribbmkpdviamegektoblvevjrvvtnukmvwpgmvlakexynnsbahgvwobsejivgbgljhikxduxrvacqqdkpighfhkyulkkysjlgmtmsljjosihvynmcmlcaqsnhvfjbxxeqtprquvywnydkpqvkagavwjnwyfnhaziijywohjuuovlviiob","ieonegohvqdijnjwziloffltzumageoynbiyureqtezkblpbdvmvbacijbwfenzdgoyvqvsyckhaoksmbkkrjmjjynskmglfvoskbwtxtaiwlkdwyabmjvhjphkuayreqhakvghmztcupximpxprqbejwxbjuhfiudwjvlsilpaegmtqvsrqanuvkvdlasvbknhrglakxiqcvvlaajfzxvkdiagnvkdqgdyqkyuuvwupoxmiejlvgxvyon","cnuwisaiyygvoldafjjskyvnnxocuqkrmleibiuxdcjgvuzxthbvoyxvzpnlzmemwbmlisfyogjpelpxejbbujywekojaknwlvrahnmaieeqkdhksovshqlaiinbyvvqewguatoapfqklkbhyvtwvknjudkqjodfktsvmajbfbxkvgmqvkheljvwurrldgzampamdgawzvkvbnfibqojeiazlvqtkgpyygvgushkuxhcpdqjdraiirtmxv","rabsbjkkutonymdmwlidwviymobvrvhkgvvejuqaroliatyktheuqviohkkcboznpnubnwkltbuzjfxvzjcjxspovpdkgkppxtdumbnapvnyeykqhqhglkcveqfshkhlglvevnzcmygdxjgavunweaiixvyjvqfkmvedvjlmxqjrumfjxjxoksvbvququbaanzdawqjodaeylratyzpissblfwiamkvgliwgigeigaiabwofeklhsydjmr","jxogakoubegqvlcdrmyikhizvevujigkqvlhmundekpoikwebglllmnwwkabyalsjvxpnugwdsfvqahmmuiyaazrmelnqyelevlnqvjvgkvgcovjewghaxoshrmbaytltiehqnjsmxjpxctklxjkkoxfwpbikqpzrvaguhrzamumvdbfbikcvzodhfvyyugtjznisiiuqovxbnuryvjnkidytkjvpvbeatwbkssdbydkdavjfpqqawajof","ieonegohvqdijnjwziloffltzumageoynbiyureqtezkblpbdvmvbacijbwfenzdgoyvqvsyckhaoksmbkkrjmjjynskmglfvoskbwtxbaiwlkdwyatmjvhjphkuayreqhakvghmztcupximpxprqbejwxbjuhfiudwjvlsilpaegmtqvsrqanuvkvdlasvbknhrglakxiqcvvlaajfzxvkdiagnvkdqgdyqkyuuvwupoxmiejlvgxvyon","xokkxyljivqbbiygkgmfndwvxuqblyamlwqosoqdrbeszrebkimgyvqytcdrvjkoiyrwvixeilstggjtvfvkbkmjkwlknkapumlhdzcvzhjzeaiigahhvasawhuqikeubavfnpbaoilorukmpftxeadfjjvpazxkacthvgvxxynjjravlidcjsousbzewmpqgehlvounvvpyvvieushqgfelnvlgumaajdbmounnwqkdwtqyjkdkpynmjb","uqijsmtvriacoqfdlotnpbrokmkehnleqqkgeuvkvmjwpvlglvtgdkwsbiivlxxmjcraebdjswdquinzyvfkogvhfhujwmfsdlyidunpyrvqaqlvilknzvkectskiwieijpxbyfoedyaugyvmwzeqvuwakjgnbepvejnowinhzukajmaaxymgvhzovdabqgamltxtkzhklbggapvfmybjqyuhbhsobjxolxubaikxvasvvayjkpkjcnrrd","ogjiviqmvgkgejrvvzkokjoudnxilycihrsxvvvyeaykghpkmwtttjrsiwnekykjjbkmidvqwjfpvaouvoowoxaqywdivghqbkqyhgflvfhlpsbmvmjuhcczflqbgmneloyvnjzdxlzpnplxseticqlbxiuevkxnuikadzjgmaenzkuvjhuagqvmblybifybkawjwuqpaupshtdmeqdlbvfkektrvuagsrdnajlebbaaivnsykomxarwad","xnvkvtideprgzhqgbyzaobfqkkzkykawzssheivkukpugmamjgqkeffmyvjxlcatwxkhvvaskdnqvvjkginkaauihwtvpaiipdnugxboyjsljmngbxioryliaglqquukerywwboirwfhejyfmorsybbdaxagclnoobvlmmkjhnjwpivbmxynnfeuluvdevsuvjvilvdzavekjvxyvwrqltdjjtjlqmcbhqboieaqpukpsacoddtzvlemhg","olwrmjdvzgnsinneshaqfwqbmqezrmbvvykbdqqegntsxoyyktwbxnbqvbhaubkmgtkjqjugkaelribjeaxaciepkmbfdhjqmvnlevykdsnrvujluvaxyykuphoyoakmaxwobzpivilvtoifealvanjovtgkjlqjswrjpvfvhkvilkrtmwikigpmdkiogzqygycigaxvdvzxxpadudnudylujjhmhfkclcavavpvswjlubeioukwfhgesz","riellsqyeiuqzjpapwvmpqyezartjhflvjwuqkvbfjuydbitkwypnnbksjbtjbaqiuazlewvdwbxevlndxpilqgvmoyavavgaomyiokxuajyhagixcjdezsrvvakfsjknqehkhvkvmnmwjgmevqhrnkpawlstkxfvoyudainivhgkbbdpditucrhtqehbxiagraxuikldnzlukvxfnkkjgmlksmsgoqvdmebvjgyclboomvwvfyogojucz","byvhyvyungwwwycoeunjojnjliifvxqqrkvgksidmsedjkzaxiekxwwluqjkuhcavldovgakvtvfjfakcxdwjpgdvymtmhojaahaomkytkebxsbpaquujxnfvevhmgmklagtmzrvekalhqeknjivssqbweifzxatvxvgqqikvjiphlbkzqjkvvgibyntvnmdujgprvfgoesrpmldsprnbzbldqyzobirbkaunpdmuleooiawcyuillhaby","dbtxpbqtbhkjgazvfiqvbmflasopmjibejdiuayizylxvrvxykqawrygjtlvshyaiwlbnuajzpzblrhigxedbmvlmveftggvhnfusveitvvvojelvznwfmuirkkbqmwakubekcxjqnfpknnoavauskueyavphcsyowqxlccdaeykptkiqdiueawvokolnsqgkkvxkjbvoqdjmghnmdmjaggjwzjoykiqwravgdnmlokudjvyhiselhurxp","ogjiviqmvgagejrvvzkokjoudnxilycihrsxvvvyeaykghpkmwtttjrsiwnekykjjbkmidvqwjfxvaouvoowoxaqywdivghqbkqyhgflvfhlpsbmvmjuhcczflqbgmneloyvnjzdxlzpnplxseticqlbxiuevkxnuikadzjgmaenzkuvjhuagqvmblybifybkawjwuqpaupshtdmeqdlbvfkektrvuagsrdnajlebbaaivnsykompkrwad","tektvgygkafuuybalsmvkubguhmaqqjmhjxivxuawlzokebamyucpqambqlswddoqyilhyasdimjnejwbvknaiytuklooqxmbvvwvzklpjizumgdwkmvvneoqgajhacwaxsbvzovhjxjfwirtdejynsfklarbhvkgpvizrrielyveklmnhvputqvipxaqfkbkeduboltjgvnbypvjrprjsvvlnedcidywggghfxnzsoeonkijxcikkdafq","wxvyqeidovwhvfscbgvylakunmtjsbnanepehfkkkvqmajaskmidphmzxpkbkgjxvtalkdsaupybtamcvrwkmrvqbkeojllrtpsigdhaqluueozjvsaajnayljqgzvbajlbknbykdzixegbnetivcrvbwayvejtfycddoynowvfouwlqkirkqeivahugyqvpziloxvgijbuudlnqymdulkkzmhohpjwrhvfnxvwfimqiuxgxigvjgoesmj","ueevkksbxmxvdutenusguauvloqdzonkrkqyvrlaithgeluavkhpxqaaaoysrpezajvkmpdatuvqyksmhvapkjnmgxvuyuxjlbwkrbfvkwlqrtepbbjzyovfkqvmifajygqcinbfgdeasmzjaxibliqogbvwndhfgiakygabxdhovmghqskmyvzpuivwqknmwlihnkzojpyjsjrjvkclewinwgjljccljoeilndtbovhdxydmwbvtvifei","ikaiaihivqfvwqixouhzwkaidlvksgyhdnrlzyvbqzabwuvjelpnakqnevkuaalejyhbymsxlsnpqxynvvlhhikqctrswduvmvfqjmmyhoaoxwpiaqdueklgsdosnbvkbmjdmcawfregiuikbvbfjokjylkugqjxesgjigvpvwouggajtlobrviennmxkpdoreftkokvxhjbyemlakwjgmgavdvrtbztcdbyvvzujxtzcepnfjmyalkupq","wdjbpfcewhuvjfqdyqeomfptuqxdtjiwwryjalbitkijkhkajglxgawkvesuydghfzxzirzgydyabooitxobgldmsfchxinipvaessorcxayxbdvvhnouqtkvbuyneywdhldhvkqsuwingpugklemgmvacrkekvpvnaulablvaumaabjvrevjovvrjtlvmifxnyneapwupvjnkmibzqvhkqbqykzavqlgkbqvljizesokmsjlmmoknjigk","mxsoakouqvnkljjnwmfhgrvoiqywsenlylepvzftaetslggakoeokkuxviicwhpbgijnlmddqehdeipbvxmhkymgvdlrvbkuvjgibbbibgfpnqabmwjolbwitfxazdhujmkvgmnunzxlvmkxpedukohwyevdijsjjodfrcyukqylwkqjevsbvzrrzviaqnuvyqsxgvvhjhgkaycvteqliaukaasmajqrxykatiwcolptpjabvykfauznvd","gvmooiuektkrevaxmiwbjaojashkbbmaglbmayhvqnvtwnydcvjgyppuuklvejqjrcbyzviabemqqntijvyglvxpjvkieuaydvgzzljvubfasavxxwvaugyomwpxjubksqkaveqdzrsvaxuwkdkdiqgxmnhwwpliabdondvjvflmsykljmeluhhbhtkegizqvhilkcomnpnqrkriufiwydjkfghgofnvcbjlfltyasrsqeodikokepxnzt","xnvkvtideprgzhqgbyzaobfqkkzkykawzssheivkukpugmamjgqkeffmyvjxlcatwxkhvvaskdnqhvjkginkaauihwtvpaiipdnugxboyjsljmngbxioryliaglqquukerywwboirwfhejyfmorsybbdaxagclnoobvlmmkjhnjwpivbmxynnfeuluvdevsuvjvilvdzavekjvxyvwrqltdjjtjlqmcbhqboieaqpukpsacoddtzvlemvg","fmjfhjgqitfbarfmehsgdgzmjkidukgkdobakbqybwqvohgshzbcemvwkvvqaitexovbjehlynuvvqeeqskswjrkmbwxmvualkfvbljgvxahkxnulikkkqirnpdupamhalpmwaodvxyejadvujsiriqglaagdaiyymailmjpfzvovnvyoosdllnwkjqglweyvrrhvestvpowccubnnxujjyxnqkuaicbbvzxkiiztpduvketljpyogyznt","djpeuvljthgqtuubiyvwoewgkjaevcmyurkubmcfxfakqwdueadtcvmajizpikvsmbvvihtosovzqxivqijsjndvayvkrgniqzpbamyjjanykktusjxjayonvygbahwjdsyshfzjlxwevxlnpwezlurokpbklgviioxszhvkmngqvgaxlfkokkxpbymarkeloolnabelmfrvdgaarkghypvbcdbvekmnifmuqlhdvjditiqbewhlqwnuqg","nbqdjymukphaefwnymugyovgauqixvwymjbjvkgninvcbvvxosjlmwmzdivwtfoydlgypilkwbgkaclyuviekhevidoolwbpveejxkqqsjeggisgkunvjkbmrlgthnofsqlvvpsaxopbltikaseruricezkmwazvfnqqvvgzfnsnaajdydduavozifjbbvxhdahylchjuxepjmxlujdrupwtkbibkyjxqkrlaraohhemzkktqkatmqviav","pefbijvvskonrajtmqwguektpjluvyykdvgjmakhoyeqweyfvbxhfvalsvkxgupwudlqlrnivtnztilxndvgrkyawkndloeypsmkjpdollmsikkmkuvjjzlagregagkxecvfhiygjdbhhmafxjiobmpaimvvobmeebykjekvqwauaqbvwtldsbisolbvwhqrziojvqzxvnoidqbaungrhmnqyicbjkaaatzjvygdcuipnqusfuxczwvxkh","arksvpqntqboagupaduhgjmqrvqgesljktgxllciygmlzqbuatoxrvjeajvybiznvvyvhajfvkclqvpnezijaocwooykbwnzkpikqhmnmadgilvvhnwyjyujjqgakfkwkxrdvuasswoleutoklgmfrixiypumlkwidlypkgepavvnkqbxsbwxsmznrtomcubyjiheblvkbnedeovfuvhkhvuqffdmiaixydevzkjgtbjdjvamwbeshadxi","sbeilghalvxbygvnpihgotwmyrxvgbhnlnqmkmsbavhaoivvnvdjqqvrueykjyktpwiiyuidqyxmuiwdcrjeizljekrhldfiapvzmopfekuokldbzxgvucvjnrynolhfdkxmgwpojcvtatalegfkgcgvakmqsamuwfqbovzyxjnevtbjsqioabljdujdwjvhvehkdksnemgskavafbuqvkseaqipbxkzlauijwwktqyzrjvykloaxupbnm","sobuexbmbiaxqmvviodjbjvpxkfvxnjejsvsdbnyktlakcutkynieokevehopvfgtlyzgwbayskqzvxhbaejlvvwlmrvkidxrnbybdandjyrjqndqvdsgainzgrqzsaeweflpjokhugujwavuuijwtfitfoyprmmwqtogahulsgommnvjyhraplkkvaqgdkeamzupkifavqvhiyakjpxhnkqikyquvzlovvwidcmlhbejgclxiuglmcbkw","riellsqyeiuqzjpapwvmpqyezartjhflvjwuqkvbfjuydbitkwypnnbksjbtjbaqiuazlewvdwbxevlndxpmlqgvmoyavavgaomyiokxuajyhagixcjdeisrvvakfsjknqehkhvkvmniwjgmevqhrnkpawlstkxfvoyudaznichgkbbdpditucrhtqehbxiagraxuikldnzlukvxfnkkjgmlksmsgoqvdmebvjgyclboomvwvfyogojuvz","xokkxyljivqbbiygkgmfndwvxuqblyamlwqosoqdrbeszrebkimgyvqytcdrvjkoiyrwvixeilstggjtvfvkbkmjkwlknkapumlhdzcvzhjzeaiigahhvasawhuqikeubavfnpbaoilorukmpfqxeadfjjvpazxkacthvgvxxynjjravlidcjsousbzewmpqgehlvounvvpyvvieushqgfelnvlgumaajdbmounnwqkdwttyjkdkpynmjb"];
        let strs = orig.iter().map(|s| s.to_string()).collect();
        let result = Solution::num_similar_groups(strs);
        assert_eq!(result, 52);
    }

    #[test]
    fn same() {
        let orig = vec!["abc","abc","def","abc","def"];
        let strs = orig.iter().map(|s| s.to_string()).collect();
        let result = Solution::num_similar_groups(strs);
        assert_eq!(result, 2);
    }

    #[test]
    fn real_world() {
        let orig = vec!["zkhnmefhyr","ykznfhehmr","mkhnyefrzh","zkhnyefrmh","zkmnhefhyr","ykznhfehmr","zkynhfehmr","mkhnhefrzy","zkhnmefryh","zkmnhfehyr"];
        let strs = orig.iter().map(|s| s.to_string()).collect();
        let result = Solution::num_similar_groups(strs);
        assert_eq!(result, 1);
    }

}