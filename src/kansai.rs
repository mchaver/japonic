use standard;
use standard::{VerbStem};

use std::cmp::Ordering::{Equal,Greater,Less};

// use std::str::FromStr;

// ひょうじゅんご 標準語

// 関西弁　かんさいべん

/*
行かない　行かへん　行けへん　行かへんかった　行けへんかった
買わない　買わへん　買えへん　変われんかった　買えへんかった
食べない　食べへん　食べれへん　食べへんかった　食べれへんかった
わからない　わからへん　　　　　わからへんかった
できる　できへん　　　　　　　　できへんかった

だ　　　だった　　　だったら
や　　　やった　　　やったら

よ　　　だよ
で　　　やで

いいよ
ええで

行くよ
行くで

そうだよ
そうやで

違う
ちゃう

食べてはいけない
食べたらあかん

食べたらダメ
食べたらあかん

食べるのはダメ
食べたらあかん

面白い　　おもろい
面白くない　おもろない
面白くなかった　おもろなかった


いる　おる
いない　おらへん
いた　おった
いなかった　おらへんかった
いて　おって

じゃない　やあらへん、やない、とちゃう



    五段 godan verbs: 使う /tukau/ ("to use") becomes 使わん /tukawaN/ and 使わへん /tukawaheN/, 使えへん /tukaeheN/
    上一段 kami-ichidan verbs: 起きる /okiru/ ("to wake up") becomes 起きん /okiN/ and 起きやへん /okijaheN/, 起きへん /okiheN/, 起きひん /okihiN/
        one mora verbs: 見る /miru/ ("to see") becomes 見ん /miN/ and 見やへん /mijaheN/, 見えへん /meːheN/, 見いひん /miːhiN/
    下一段 shimo-ichidan verbs: 食べる /taberu/ ("to eat") becomes 食べん /tabeN/ and 食べやへん /tabejaheN/, 食べへん /tabeheN/
        one mora verbs: 寝る /neru/ ("to sleep") becomes 寝ん /neN/ and 寝やへん /nejaheN/, 寝えへん /neːheN/
    s-irregular verb: する /suru/ becomes せん /seN/ and しやへん /sijaheN/, せえへん /seːheN/, しいひん /siːhiN/
    k-irregular verb: 来る /kuru/ becomes 来ん /koN/ and きやへん /kijaheN/, けえへん /keːheN/, きいひん /kiːhiN/
        来おへん /koːheN/, a mixture けえへん /keːheN/ with standard 来ない /konai/, is also used lately by young people, especially in Kobe.

if v1, length == 1 and 
*/

pub const HIRAGANA_TO_VOWEL_TABLE: &'static [(&str, &str)] = &[
    ("あ","あ"),("か","あ"),("さ","あ"),("た","あ"),("な","あ"),("は","あ"),("ま","あ"),("や","あ"),("ら","あ"),("わ","あ"),("が","あ"),("ざ","あ"),("だ","あ"),("ば","あ"),("ぱ","あ"),("ゃ","あ"),
    ("い","い"),("き","い"),("し","い"),("ち","い"),("に","い"),("ひ","い"),("み","い"),("り","い"),("ぎ","い"),("じ","い"),("ぢ","い"),("び","い"),("ぴ","い"),
    ("う","う"),("く","う"),("す","う"),("つ","う"),("ぬ","う"),("ふ","う"),("む","う"),("ゆ","う"),("る","う"),("ぐ","う"),("ず","う"),("づ","う"),("ぶ","う"),("ぷ","う"),("ゅ","う"),
    ("え","え"),("け","え"),("せ","え"),("て","え"),("ね","え"),("へ","え"),("め","え"),("れ","え"),("げ","え"),("ぜ","え"),("で","え"),("ば","え"),("べ","え"),
    ("お","お"),("こ","お"),("そ","お"),("と","お"),("の","お"),("ほ","お"),("も","お"),("よ","お"),("ろ","お"),("を","お"),("ご","お"),("ぞ","お"),("ど","お"),("ぼ","お"),("ぽ","お"),("ょ","お")
];

fn lookup_table(s: &str, table: &'static [(&str,&str)]) -> Option<usize> {
    match table.binary_search_by(|&(key, _)| {
        if s == key { Equal }
        else if key < s { Less }
        else { Greater }
    }) {
        Ok(i) => Some(i),
        Err(_) => None,
    }
}

pub enum VerbType {
    V1Iru, // いる　上一段　かみいちだん
    V1Eru, // える　下一段　しもいちだん
    Standard(standard::VerbType)
}

pub enum VerbConjugation {
    NonPastNegative,
    PastNegative,
    NonPastPotentialNegative,
    PastPotentialNegative
}

// pub enum VerbStem = standard::VerbStem;

pub fn verb_stem(word: &str, verb_stem: standard::VerbStem, verb_type: VerbType) -> String {
    match verb_type {
        VerbType::V1Iru => standard::verb_stem(word,verb_stem,standard::VerbType::V1),
        VerbType::V1Eru => standard::verb_stem(word,verb_stem,standard::VerbType::V1),
        VerbType::Standard(vt) => standard::verb_stem(word,verb_stem,vt)
    }
}
    
pub fn conjugate_verb(verb: &str, vt: VerbType, conjugation: VerbConjugation) -> Option<String> {
    match conjugation {
        //VerbConjugation::NonPastNegative => Some(format!("{}{}", standard::verb_stem(verb, standard::VerbStem::Irrealis, vt), "へん")),

        VerbConjugation::NonPastNegative => 
            match vt {
                VerbType::V1Iru => {
                    let verb_stem = standard::verb_stem(verb, VerbStem::Irrealis, standard::VerbType::V1);
                    if verb_stem.chars().count() == 1 {
                        Some(format!("{}{}", verb_stem, "いへん"))
                    } else {
                        Some(format!("{}{}", verb_stem, "へん"))
                    }                
                },
                VerbType::V1Eru => {
                    let verb_stem = standard::verb_stem(verb, VerbStem::Irrealis, standard::VerbType::V1);
                    if verb_stem.chars().count() == 1 {
                        Some(format!("{}{}", verb_stem, "えへん"))
                    } else {
                        Some(format!("{}{}", verb_stem, "へん"))
                    }                
                },
                VerbType::Standard(vt) => Some(format!("{}{}", standard::verb_stem(verb, VerbStem::Irrealis, vt), "へん")),
                
            },

        _ => None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_conjugate_verb() {
        assert_eq!(conjugate_verb("いく",VerbType::Standard(standard::VerbType::V5KS),VerbConjugation::NonPastNegative), Some("いかへん".to_string()));
        assert_eq!(conjugate_verb("行く",VerbType::Standard(standard::VerbType::V5KS),VerbConjugation::NonPastNegative), Some("行かへん".to_string()));

        assert_eq!(conjugate_verb("食べる",VerbType::V1Eru,VerbConjugation::NonPastNegative), Some("食べへん".to_string()));
        assert_eq!(conjugate_verb("見る",VerbType::V1Iru,VerbConjugation::NonPastNegative), Some("見いへん".to_string()));
        assert_eq!(conjugate_verb("寝る",VerbType::V1Eru,VerbConjugation::NonPastNegative), Some("寝えへん".to_string()));
    }
}
