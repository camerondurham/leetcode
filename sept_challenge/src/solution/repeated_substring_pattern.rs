struct Solution {}

impl Solution {
    pub fn repeated_substring_pattern(s: String) -> bool {
        let (mut i, mut j, n) = (1, 0, s.len());
        let mut dp: Vec<i32> = vec![0; n + 1];
        while i < n {
            if s.as_bytes()[i] == s.as_bytes()[j] {
                i += 1;
                j += 1;
                dp[i] = j as i32;
            } else if j == 0 {
                i += 1;
            } else {
                j = dp[j] as usize;
            }
        }
        dp[n] > 0 && dp[n] % (n as i32 - dp[n] as i32) == 0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /*
    Given a non-empty string check if it can be constructed by taking a
    substring of it and appending multiple copies of the substring together.
    You may assume the given string consists of lowercase English letters
    only and its length will not exceed 10000.

    Example 1:

    Input: "abab"
    Output: True
    Explanation: It's the substring "ab" twice.

    Example 2:

    Input: "aba"
    Output: False

    Example 3:

    Input: "abcabcabcabc"
    Output: True
    Explanation: It's the substring "abc" four times. (And the substring "abcabc" twice.)


    abadabad
    aabddaba

        bool repeatedSubstringPattern(string str) {
        int i = 1, j = 0, n = str.size();
        vector<int> dp(n+1,0);
        while( i < str.size() ){
            if( str[i] == str[j] ) dp[++i]=++j;
            else if( j == 0 ) i++;
            else j = dp[j];
        }
        return dp[n]&&dp[n]%(n-dp[n])==0;
    }


    */

    #[test]
    fn ex1() {
        assert_eq!(
            Solution::repeated_substring_pattern("abab".to_string()),
            true
        );
    }

    #[test]
    fn ex2() {
        assert_eq!(
            Solution::repeated_substring_pattern("aba".to_string()),
            false
        );
    }

    #[test]
    fn ex3() {
        assert_eq!(
            Solution::repeated_substring_pattern("abcabcabcabc".to_string()),
            true
        );
    }

    #[test]
    fn ex4() {
        assert_eq!(
            Solution::repeated_substring_pattern("vkuzkjjbbzozfqhzmkkhezvbnwamzvgbltptzexdzudkqvyctpvagwmttywcuzyngqwxnbdpakuvreifbwyikuzrnzcntkunzwrbsbzhiclsqqkclaetqkiapdmbfqyfpkihpfembsxyrtuqupmqrgthtnkghuucqhnxyuucthbczxbxftdaoyirdfxszcumitfzzzukxfetovixeoypijkhxzeldpybsnkmrhjpmczflpvzxdevufovbdkbnfqemljfyaxrrwpwzbhwwiwgvmjanmoznyxjnfmfdopcaiuvksjmmnrvdjtywjtfalxypxwgzarsaggjfibxhcowpsfvkwmiagorfpnlluycelnngrpgnywdpfyxtczyifiyihfugwkikmnvubjzefufneiiwilfazezulwepvmtvygcoduaxmeehgexprxtwcmfqpwystrbmvzdlflmjhcocyiqlfsjydhtpwpogqmtemmlpfhjyfhcfvovhdjrjijmzszfgxiinutlfsnbiqnpdikztysnspvusyhqqqbfetlwrrboamkxccdhwhwrmnginsaeefrlwodbczyutpapqtiruscyymbpcgycytfnkxxpcxinhgiuvrurrveuekknfyvmjhivpbfgdikqikmnohduaxcmndqqazusugaaewlgujbjyklkcobxscaivpnnyqnfkafxdmoqfinojztevofjmjwurwmqduhklymwoxoetyxibhrmpegyiwmxmaqorpykjladzppythuwcgtsivwjceghnylwizbtpdhqogpuwyqmkvndyiqbmydfvgazabquwftghwesasejxgsaaxvtdfwpxrbyyplvsigwfcmsdkmkcdkgrpuheoxbdgruitsodxshnaxizebwlfuvpkukypsaiozivbjxlnyuwcwvggnjlsnblrzsgjtsjdyadqemxcqkncljtmygdbzrfpapsgfbtyibjxgsqtsxtxicgrxdhlawlwzdlnxgjtojyejtkkpqdjysgyuperosctuyuhcwmheoeadqrofsrrjtfksxwcbsliyvylkgyfmhxwldmyieybkfxfxugxlneikswlnicqzwxnkkweardfzyrxmkqcnmarroedzqhyuyinsjsfbqbcncoopxpmktljdyoibsgrtthnsntipnntdswggfzmenapjqdlqcvskfodegqrvuiuncqbksnivfaygxatfdqcvukflbcmelomvfpicohivzdglhzeabvlddgidytlrocvrbhfivalqlrzhbhcuptltnckjgnrohjwkqoinfgycnayrqrqwkqzmuoqxhcjankevwhlpwibjvfvdnfbaeiqgtyufgchxehzaleiicwbbminjxbilrungfjdygasvztrxiedvdlfcgbgutqqbuvpovuvvqcristdobriozyvylhhonxsastorkruuvtjluhvwypbtbcxuysrzznuuwmnrpkafnvzeewsibovfimthqigrkjibesianqwskwdritimugwovjkpuahnfmhsfljvrltjnbotjpwjzxnetxhljtwucvlwakazrorzrsbvfhadpqpotqyjqhwvyvwndqdysuuotfsoejokortfpgqybpwlmpjwydydjczdedvkuacyrnstazrrzaincaiahdnyizvdrmudfwlhmwxcsctrbqljvtekmcjcrtgyhtfgpaloqgfwuuezlcjsfneyhwaonekhneiqfcpjvrpphrvtzotpajhsevjeyewjutwpnpzhulbulussazygaiprppogjfainxqliqwxxfzelpjijcwzhhewznzyuerxguctggyrzitwrigvbjtreametaiaossuvtofeqgubmgucdgjmipilupqirkpiinczfhgeqtywxfrdpupgomdqcamdjwsmwtrpkhkwghizsfanlbfljsumcbwbzdtpasmiejjjcacykihkevsgzlslavsowfsfasnnsfhuundiyghflprbpnjjovikkmeitiymwmdlhtdicbxrpqeacexeriwlntqudtwymcoktnxoayytglijxxvrtwyyyxxsqgqkjhgfhcjqhqchwfqemiheddfqrnvqsgiukfseligagbchtkscnmbmrbvaecuahxwzffbvzmnktnzxvembdnjumzhcyxtappiqpnoipmpazjimxdofrwyxtetunxhdtuqvnygqsgkgerunawbhllyxirmbftbkzhjwelomogmzimlpjklhgzrvxjerzkgwkddgdxfwypxkmjleirdaacbxcasgiuwzgjzsfmeitetmozvtvukglclkvafaywacryevzjjvdmiwfpmprtsoanypmojtekfduumpuufkrfeeakwwphvqjgkvjsthnrhjxhpmpracaruxvsrdwbfhabwwhazosfqgriykenckywhqxyaqtkgdvuvjnvlpjxfmguxtgyhypvlhqnscfqbmcsalavqrgutbeejiulbcrwrmphbgenntnimmimzyvcvopinntmnqdakbhuzxexsrmnqlgxyupcjoeohpqpsgbbqjxyeoevpbovlelwwmdrikudjtmzstgkojvgikuupeeagzaqnsmuxiscqydocsodevsyiflybaqvcszgqxzsbtcsmzjdnwjoxcekytjtzskpytvqvtagibgaxgauxjedibdeijpmeedqupyswmptexhdbsqvuuhqqkpwfqpgncedzomsxiyvbiurykgjcgltsbawqdnbxcrryflmzgqgyjuwxlktzdxogtxmwczdefzxaztrmhughjtoyilliaghqqnlynvtgguinyquxtpuuycpzfofqxmmlnnmnjigidirzqgmjmkrsxgrjrquyfeyrfuuqdamomxxgghzynnlemcowhqpuuetusxwvjnitorwjztmykwhscowstsfojslrclnwoelmoguqtfmknvgjvezgmiliqabftvmxvrjeixqjvddlyqzlxdwplnxcomsvmiixlotkbstxhfkfwxilblujxropyybqvpvfpceehdbvvfzcnqnceogkgiqjrmsfdmauurxpvmgihqqyjxpifitgwokbgxvhfmvagfegbevtqztmoaaktdvinimilioihdugvifhcyenvfldosukxxulutuezajmwfskuggrumeuzklfnxtqmcyerdhobrrollwsinxfiiwcemxrpzqqpkyddfihubhgaydfzgxrcuojcfqiigxcysdhmrvgaxxdwywsykowozpgdvbhcyrxqsrzkodzadojhbynxgfczwkziqgbrpucmbduiblwtfizncsaoxptltfuamramamoyckdisptuzdzhprfltzbqmodytuyqvqdyqnpzkyggwfznzcrthstgasjphzzrzksbvkzneafsfitxcssaihvlgkhvbkogheqdqzrscgrykoxtctccsidkuetlymldxdlmzyixcxcdekpfhelspcjvcmbmvybbeusrnwtxhsxnztjxvgwnxtfixbvhzkjaqrusrinmpdhcuvykxlkibocstdbitibilfkjeupswavaplmqgoiumccurmgipmppknpgrtowqudhbpnvfwzzojilkoeywrqdqgfjjnvrvxfrvplsqrgfihyqizudqlhtluugzqlorxbwdxqklsnqbrbvymzevyoteqouvvlwexhyuwkfdmcgaykubecejepmdxfvypuhlbmytxmwcpfnwwpdggvgsdmghdxfpqhhwkcyqslewpzubdymifegfakqdhjhwekwpfqwqagdqzvfnkkwtlfnlvmpmwssrmgquwhcqpodutmjeerbcpysobphetitiugdriqnrjsdsxkgqyczfozjotakgemuczogjbfwylqqdbtvzktvwhcanyqvxqyttwrzatjomjjorbipjvpvtxdlopgajcielcqjypmvljovozgbpdhhtaeehjahcvvmhwudggpjhxhjdxvzixzjvjooosceqfcyebqmodfdtnjpvymvdjpygbaamsjsjrtwinsrafgbjsrslrcazpumbsmuuaisdiacjlysrhdlfjbqxlkdtaihdfbpzixgrvprgtesxwgpgmhxvnemykoiqrlwzclghcmywcghltqhfziokgkbvynlmckiznavxgmywgkkfacuetrlqgmufeqvflunxlnwyawzkmoconujmovkgnksabofkytdmtubriyxvhkltgnddfiejyjrziwfqwcgarqhxicutzekyhwsgphmhmiyttxenpbgbqfuixzypanchvzqovsjanjsrjatdmugavtneptptdqejharuckmcgirvnrdahjlhvacsfsozyecgsmhtoggjmdaeejmznmnatqwtoljrzcivxcjfdikymjmycabrzhdlzjxmnqgzobbszkqhobvhrutiaftetzixxwqzpmdefvacupqjeimtmtefdnxtzqmocqveljxfukjivzwupfxivviuecxihshaxyuvitjflimwbdtstxiutljvjskajqicpfndrpwlqxduelxrxxzvkuzkjjbbzozfqhzmkkhezvbnwamzvgbltptzexdzudkqvyctpvagwmttywcuzyngqwxnbdpakuvreifbwyikuzrnzcntkunzwrbsbzhiclsqqkclaetqkiapdmbfqyfpkihpfembsxyrtuqupmqrgthtnkghuucqhnxyuucthbczxbxftdaoyirdfxszcumitfzzzukxfetovixeoypijkhxzeldpybsnkmrhjpmczflpvzxdevufovbdkbnfqemljfyaxrrwpwzbhwwiwgvmjanmoznyxjnfmfdopcaiuvksjmmnrvdjtywjtfalxypxwgzarsaggjfibxhcowpsfvkwmiagorfpnlluycelnngrpgnywdpfyxtczyifiyihfugwkikmnvubjzefufneiiwilfazezulwepvmtvygcoduaxmeehgexprxtwcmfqpwystrbmvzdlflmjhcocyiqlfsjydhtpwpogqmtemmlpfhjyfhcfvovhdjrjijmzszfgxiinutlfsnbiqnpdikztysnspvusyhqqqbfetlwrrboamkxccdhwhwrmnginsaeefrlwodbczyutpapqtiruscyymbpcgycytfnkxxpcxinhgiuvrurrveuekknfyvmjhivpbfgdikqikmnohduaxcmndqqazusugaaewlgujbjyklkcobxscaivpnnyqnfkafxdmoqfinojztevofjmjwurwmqduhklymwoxoetyxibhrmpegyiwmxmaqorpykjladzppythuwcgtsivwjceghnylwizbtpdhqogpuwyqmkvndyiqbmydfvgazabquwftghwesasejxgsaaxvtdfwpxrbyyplvsigwfcmsdkmkcdkgrpuheoxbdgruitsodxshnaxizebwlfuvpkukypsaiozivbjxlnyuwcwvggnjlsnblrzsgjtsjdyadqemxcqkncljtmygdbzrfpapsgfbtyibjxgsqtsxtxicgrxdhlawlwzdlnxgjtojyejtkkpqdjysgyuperosctuyuhcwmheoeadqrofsrrjtfksxwcbsliyvylkgyfmhxwldmyieybkfxfxugxlneikswlnicqzwxnkkweardfzyrxmkqcnmarroedzqhyuyinsjsfbqbcncoopxpmktljdyoibsgrtthnsntipnntdswggfzmenapjqdlqcvskfodegqrvuiuncqbksnivfaygxatfdqcvukflbcmelomvfpicohivzdglhzeabvlddgidytlrocvrbhfivalqlrzhbhcuptltnckjgnrohjwkqoinfgycnayrqrqwkqzmuoqxhcjankevwhlpwibjvfvdnfbaeiqgtyufgchxehzaleiicwbbminjxbilrungfjdygasvztrxiedvdlfcgbgutqqbuvpovuvvqcristdobriozyvylhhonxsastorkruuvtjluhvwypbtbcxuysrzznuuwmnrpkafnvzeewsibovfimthqigrkjibesianqwskwdritimugwovjkpuahnfmhsfljvrltjnbotjpwjzxnetxhljtwucvlwakazrorzrsbvfhadpqpotqyjqhwvyvwndqdysuuotfsoejokortfpgqybpwlmpjwydydjczdedvkuacyrnstazrrzaincaiahdnyizvdrmudfwlhmwxcsctrbqljvtekmcjcrtgyhtfgpaloqgfwuuezlcjsfneyhwaonekhneiqfcpjvrpphrvtzotpajhsevjeyewjutwpnpzhulbulussazygaiprppogjfainxqliqwxxfzelpjijcwzhhewznzyuerxguctggyrzitwrigvbjtreametaiaossuvtofeqgubmgucdgjmipilupqirkpiinczfhgeqtywxfrdpupgomdqcamdjwsmwtrpkhkwghizsfanlbfljsumcbwbzdtpasmiejjjcacykihkevsgzlslavsowfsfasnnsfhuundiyghflprbpnjjovikkmeitiymwmdlhtdicbxrpqeacexeriwlntqudtwymcoktnxoayytglijxxvrtwyyyxxsqgqkjhgfhcjqhqchwfqemiheddfqrnvqsgiukfseligagbchtkscnmbmrbvaecuahxwzffbvzmnktnzxvembdnjumzhcyxtappiqpnoipmpazjimxdofrwyxtetunxhdtuqvnygqsgkgerunawbhllyxirmbftbkzhjwelomogmzimlpjklhgzrvxjerzkgwkddgdxfwypxkmjleirdaacbxcasgiuwzgjzsfmeitetmozvtvukglclkvafaywacryevzjjvdmiwfpmprtsoanypmojtekfduumpuufkrfeeakwwphvqjgkvjsthnrhjxhpmpracaruxvsrdwbfhabwwhazosfqgriykenckywhqxyaqtkgdvuvjnvlpjxfmguxtgyhypvlhqnscfqbmcsalavqrgutbeejiulbcrwrmphbgenntnimmimzyvcvopinntmnqdakbhuzxexsrmnqlgxyupcjoeohpqpsgbbqjxyeoevpbovlelwwmdrikudjtmzstgkojvgikuupeeagzaqnsmuxiscqydocsodevsyiflybaqvcszgqxzsbtcsmzjdnwjoxcekytjtzskpytvqvtagibgaxgauxjedibdeijpmeedqupyswmptexhdbsqvuuhqqkpwfqpgncedzomsxiyvbiurykgjcgltsbawqdnbxcrryflmzgqgyjuwxlktzdxogtxmwczdefzxaztrmhughjtoyilliaghqqnlynvtgguinyquxtpuuycpzfofqxmmlnnmnjigidirzqgmjmkrsxgrjrquyfeyrfuuqdamomxxgghzynnlemcowhqpuuetusxwvjnitorwjztmykwhscowstsfojslrclnwoelmoguqtfmknvgjvezgmiliqabftvmxvrjeixqjvddlyqzlxdwplnxcomsvmiixlotkbstxhfkfwxilblujxropyybqvpvfpceehdbvvfzcnqnceogkgiqjrmsfdmauurxpvmgihqqyjxpifitgwokbgxvhfmvagfegbevtqztmoaaktdvinimilioihdugvifhcyenvfldosukxxulutuezajmwfskuggrumeuzklfnxtqmcyerdhobrrollwsinxfiiwcemxrpzqqpkyddfihubhgaydfzgxrcuojcfqiigxcysdhmrvgaxxdwywsykowozpgdvbhcyrxqsrzkodzadojhbynxgfczwkziqgbrpucmbduiblwtfizncsaoxptltfuamramamoyckdisptuzdzhprfltzbqmodytuyqvqdyqnpzkyggwfznzcrthstgasjphzzrzksbvkzneafsfitxcssaihvlgkhvbkogheqdqzrscgrykoxtctccsidkuetlymldxdlmzyixcxcdekpfhelspcjvcmbmvybbeusrnwtxhsxnztjxvgwnxtfixbvhzkjaqrusrinmpdhcuvykxlkibocstdbitibilfkjeupswavaplmqgoiumccurmgipmppknpgrtowqudhbpnvfwzzojilkoeywrqdqgfjjnvrvxfrvplsqrgfihyqizudqlhtluugzqlorxbwdxqklsnqbrbvymzevyoteqouvvlwexhyuwkfdmcgaykubecejepmdxfvypuhlbmytxmwcpfnwwpdggvgsdmghdxfpqhhwkcyqslewpzubdymifegfakqdhjhwekwpfqwqagdqzvfnkkwtlfnlvmpmwssrmgquwhcqpodutmjeerbcpysobphetitiugdriqnrjsdsxkgqyczfozjotakgemuczogjbfwylqqdbtvzktvwhcanyqvxqyttwrzatjomjjorbipjvpvtxdlopgajcielcqjypmvljovozgbpdhhtaeehjahcvvmhwudggpjhxhjdxvzixzjvjooosceqfcyebqmodfdtnjpvymvdjpygbaamsjsjrtwinsrafgbjsrslrcazpumbsmuuaisdiacjlysrhdlfjbqxlkdtaihdfbpzixgrvprgtesxwgpgmhxvnemykoiqrlwzclghcmywcghltqhfziokgkbvynlmckiznavxgmywgkkfacuetrlqgmufeqvflunxlnwyawzkmoconujmovkgnksabofkytdmtubriyxvhkltgnddfiejyjrziwfqwcgarqhxicutzekyhwsgphmhmiyttxenpbgbqfuixzypanchvzqovsjanjsrjatdmugavtneptptdqejharuckmcgirvnrdahjlhvacsfsozyecgsmhtoggjmdaeejmznmnatqwtoljrzcivxcjfdikymjmycabrzhdlzjxmnqgzobbszkqhobvhrutiaftetzixxwqzpmdefvacupqjeimtmtefdnxtzqmocqveljxfukjivzwupfxivviuecxihshaxyuvitjflimwbdtstxiutljvjskajqicpfndrpwlqxduelxrxxz".to_string()),
            true
        );
    }
}
