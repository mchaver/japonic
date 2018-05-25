use standard::tables::{ROMAJI_TO_HIRAGANA_TABLE};
use std::str::FromStr;
pub use standard::util::romaji_to_hiragana;

#[derive(Debug, PartialEq)]
pub enum VerbType {
    V1,
    V5Aru,
    V5B,
    V5G,
    V5K,
    V5KS,
    V5M,
    V5N,
    V5R,
    V5RI,
    V5S,
    V5T,
    V5U,
    V5US,
    VK,
    VS
}

impl FromStr for VerbType {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "v1"    => Ok(VerbType::V1),
            "v5aru" => Ok(VerbType::V5Aru),
            "v5b"   => Ok(VerbType::V5B),
            "v5g"   => Ok(VerbType::V5G),
            "v5k"   => Ok(VerbType::V5K),
            "v5k-s" => Ok(VerbType::V5KS),
            "v5m"   => Ok(VerbType::V5M),
            "v5n"   => Ok(VerbType::V5N),
            "v5r"   => Ok(VerbType::V5R),
            "v5r-i" => Ok(VerbType::V5RI),
            "v5s"   => Ok(VerbType::V5S),
            "v5t"   => Ok(VerbType::V5T),
            "v5u"   => Ok(VerbType::V5U),
            "v5u-s" => Ok(VerbType::V5US),
            "vk"    => Ok(VerbType::VK),
            "vs"    => Ok(VerbType::VS),
            _       => Err(s.to_string())
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum VerbStem {
    Dictionary,   // 辞書形, u form
    Conjunctive,  // 連用形, i form
    Irrealis,     // 未然形, a form
    Imperative,   // 命令形, e form
    Hypothetical, // 推量形, o form
    Te,           // テ形, te form
    Ta,           // タ形, ta form
}

#[derive(Debug, PartialEq)]
pub enum VerbConjugation {
    // 辞書形　Dictionary
    // NonPast
    // な negative imperative NegativeImperative
    // の emphatic/informal Emphatic
    // こと nominalize Nominalize
    // ことができる able to do PotentialI
    // ことがある　occasional occurence Occasional
    // ことにする decide to do Decisive
    // ことになる as a result Resultative
    // まえに before ~ing Before
    // がはやいか as soon as AsSoonAs
    // ともなく without intent Unintentionally
    // べきだ idealistic should IdealisticalShould
    // まい formal negative volitional NegativeVolitionalFormal
    // みたい it seems that EvidentialI
    // そう I've heard that EvidentialII
    // らしい apparently い-adj EvidentialIII
    NonPast,

    // 連用形　conjunctive
    // たい　desire い-adj DesireI
    // たがる　desire 五段動詞 DerireII
    // はしない strong negative desire DesireNegative
    // ながら while ~ing While
    // がち tends to TendsTo
    // かた way of ~ing WayOf
    // そう it looks like EvidentialIV
    // やがる yakuza super rude Rude
    // にくい difficult to DifficultTo
    // やすい easy to EasyTo
    // すぎる too much TooMuch
    // に+verb of motion go and do Directional
    // polite
    // ます non past NonPastPolite
    // ません negative NonPastNegativePolite
    // ました past PastPolite
    // ませんでした past negative PastNegativePolite
    // ましょう volitional Volitional
    // まして connective (rare) ConnectivePolite
    // ますれば conditional (rare) ConditionalPolite
    // なさい imperative ImperativeI
    // な　imperative ImperativeII
    // お+連用形+になる honorific verb form HonorificI
    // お+連用形+なさる honorific verb form HonorificII
    // お+連用形+する humble verb form Humble

    While,
    DifficultTo,
    EasyTo,
    TooMuch,
    NonPastPolite,
    NonPastNegativePolite,
    PastPolite,
    PastNegativePolite,
    VolitionalPolite,
    HonorificI,
    HonorificII,
    Humble,

    // 未然形
    // ない non past negative い形容詞
    // ないで without ing
    // なくて negative テ形
    // なかった　past negative タ形
    // なければ　negative conditional
    // なかろう negative volitional
    // なくていい don't have to (informal)
    // なくてはいけない must (compulsion)
    // なくちゃ must (compulsion)
    // なければならない must (obligation)
    // なきゃ must (obligation)
    // なかればいけない must (obligation)
    // ず negative (old)
    // ずに without ~ing (formal)
    // れる　五段 passive/honorific
    // られる 一段 passive/honorific
    // せる/す　五段 caustive
    // させる/さす 一段　causative
    // せられる/される 五段 causative passive
    // させられる 一段　caustive passive

    NonPastNegative,
    PastNegative,
    Passive,
    Causative,
    CausativePassive,

    // 命令形
    // ば 五段 conditional
    // れば　一段 conditional
    // る　五段 potential
    // られる/れる 一段 potential
    // ろ 一段　imperative
    // conditional + よかった I wish that

    Conditional,
    
    // 推量形
    // う 五段 volitional/tenative/presumptive
    // よう 一段 volitional/tenative/presumptive

    Volitional,
    
    // テ形
    // あげる・くれる・もらう benefit being given or received
    // いく continuing change of state
    // いる continuous/habitual action
    // おく preparatory action
    // しまう completed acttion
    // よかった I'm glad that
    // みる try and see
    // ほしい favor request
    // ある changed state
    // から after ~ing
    // くる state change
    // はいけない must not
    // はならない must not
    // はだめ must not
    // もかまわない permissive
    // もいい permissive
    // も even if
    // すみません apologetic

    Try,

    // タ形
    // から reason for next clause
    // りする etc., repeated opposite
    // ら if/when ...
    // ばかり just happened
    // ほうがいい suggestive

    Just
}

pub fn verb_stem(word: &str, verb_stem: VerbStem, verb_type: VerbType) -> String {
    if word.len() > 0 {
        let s = truncate_chars(word, (word.chars().count() - 1)).to_string();

        match (verb_type,verb_stem) {
            (_, VerbStem::Dictionary) => word.to_string(),

            (VerbType::V1,VerbStem::Conjunctive) => s + "ない",
            (VerbType::V1,VerbStem::Irrealis)    => s,
            //            (VerbType::V1,VerbStem::Imperative)  => (s + "よ").to_string(),
            (VerbType::V1,VerbStem::Imperative)  => s,
            (VerbType::V1,VerbStem::Hypothetical) => s , // + "れ",
            (VerbType::V1,VerbStem::Te)          => s + "て",
            (VerbType::V1,VerbStem::Ta)          => s + "た",
            
            (VerbType::V5Aru,VerbStem::Conjunctive)  => s + "り",
            (VerbType::V5Aru,VerbStem::Irrealis)     => "ない".to_string(),
            (VerbType::V5Aru,VerbStem::Imperative)   => s + "れ",
            (VerbType::V5Aru,VerbStem::Hypothetical)  => s + "ろ",
            (VerbType::V5Aru,VerbStem::Te)           => s + "って",
            (VerbType::V5Aru,VerbStem::Ta)           => s + "った",

            (VerbType::V5B,VerbStem::Conjunctive)  => s + "び",
            (VerbType::V5B,VerbStem::Irrealis)     => s + "ば",
            (VerbType::V5B,VerbStem::Imperative)   => s + "べ",
            (VerbType::V5B,VerbStem::Hypothetical)  => s + "ぼ",
            (VerbType::V5B,VerbStem::Te)           => s + "んで",
            (VerbType::V5B,VerbStem::Ta)           => s + "んだ",

            (VerbType::V5G,VerbStem::Conjunctive)  => s + "ぎ",
            (VerbType::V5G,VerbStem::Irrealis)     => s + "が",
            (VerbType::V5G,VerbStem::Imperative)   => s + "げ",
            (VerbType::V5G,VerbStem::Hypothetical)  => s + "ご",
            (VerbType::V5G,VerbStem::Te)           => s + "いで",
            (VerbType::V5G,VerbStem::Ta)           => s + "いだ",

            (VerbType::V5K,VerbStem::Conjunctive)  => s + "き",
            (VerbType::V5K,VerbStem::Irrealis)     => s + "か",
            (VerbType::V5K,VerbStem::Imperative)   => s + "け",
            (VerbType::V5K,VerbStem::Hypothetical)  => s + "こ",
            (VerbType::V5K,VerbStem::Te)           => s + "いて",
            (VerbType::V5K,VerbStem::Ta)           => s + "いた",

            (VerbType::V5KS,VerbStem::Conjunctive)  => s + "き",
            (VerbType::V5KS,VerbStem::Irrealis)     => s + "か",
            (VerbType::V5KS,VerbStem::Imperative)   => s + "け",
            (VerbType::V5KS,VerbStem::Hypothetical)  => s + "こ",
            (VerbType::V5KS,VerbStem::Te)           => s + "って",
            (VerbType::V5KS,VerbStem::Ta)           => s + "った",

            (VerbType::V5M,VerbStem::Conjunctive)  => s + "み",
            (VerbType::V5M,VerbStem::Irrealis)     => s + "ま",
            (VerbType::V5M,VerbStem::Imperative)   => s + "め",
            (VerbType::V5M,VerbStem::Hypothetical)  => s + "も",
            (VerbType::V5M,VerbStem::Te)           => s + "んで",
            (VerbType::V5M,VerbStem::Ta)           => s + "んだ",

            (VerbType::V5N,VerbStem::Conjunctive)  => s + "に",
            (VerbType::V5N,VerbStem::Irrealis)     => s + "な",
            (VerbType::V5N,VerbStem::Imperative)   => s + "ね",
            (VerbType::V5N,VerbStem::Hypothetical)  => s + "の",
            (VerbType::V5N,VerbStem::Te)           => s + "んで",
            (VerbType::V5N,VerbStem::Ta)           => s + "んだ",

            (VerbType::V5R,VerbStem::Conjunctive)  => s + "り",
            (VerbType::V5R,VerbStem::Irrealis)     => s + "ら",
            (VerbType::V5R,VerbStem::Imperative)   => s + "れ",
            (VerbType::V5R,VerbStem::Hypothetical)  => s + "ろ",
            (VerbType::V5R,VerbStem::Te)           => s + "って",
            (VerbType::V5R,VerbStem::Ta)           => s + "った",

            (VerbType::V5RI,VerbStem::Te)  => s + "って",

            (VerbType::V5S,VerbStem::Conjunctive)  => s + "し",
            (VerbType::V5S,VerbStem::Irrealis)     => s + "さ",
            (VerbType::V5S,VerbStem::Imperative)   => s + "せ",
            (VerbType::V5S,VerbStem::Hypothetical)  => s + "そ",
            (VerbType::V5S,VerbStem::Te)           => s + "して",
            (VerbType::V5S,VerbStem::Ta)           => s + "した",

            (VerbType::V5T,VerbStem::Conjunctive)  => s + "ち",
            (VerbType::V5T,VerbStem::Irrealis)     => s + "た",
            (VerbType::V5T,VerbStem::Imperative)   => s + "て",
            (VerbType::V5T,VerbStem::Hypothetical)  => s + "と",
            (VerbType::V5T,VerbStem::Te)           => s + "って",
            (VerbType::V5T,VerbStem::Ta)           => s + "った",

            (VerbType::V5U,VerbStem::Te)   => s + "って",
            (VerbType::V5US,VerbStem::Te)  => s + "うて",
            (VerbType::VK,VerbStem::Te)    => s + "て",
            (VerbType::VS,VerbStem::Te)    => "して".to_string(),

            (VerbType::V5U,VerbStem::Ta)   => s + "った",
            (VerbType::V5US,VerbStem::Ta)  => s + "うた",
            (VerbType::VK,VerbStem::Ta)    => s + "た",
            (VerbType::VS,VerbStem::Ta)    => "した".to_string(),
            
            (_,_) => s

        }
    } else {
        word.to_string()
    }
}


// assumes that verb is in dictionary form
pub fn conjugate_verb(verb: &str, vt: VerbType, conjugation: VerbConjugation) -> Option<String> {
    match conjugation {
        // 辞書形
        VerbConjugation::NonPast => Some(verb.to_string()),

        // 連用形
        VerbConjugation::While                 => Some(format!("{}{}", verb_stem(verb, VerbStem::Conjunctive, vt), "ながら")),
        VerbConjugation::DifficultTo           => Some(format!("{}{}", verb_stem(verb, VerbStem::Conjunctive, vt), "にくい")),
        VerbConjugation::EasyTo                => Some(format!("{}{}", verb_stem(verb, VerbStem::Conjunctive, vt), "やすい")),
        VerbConjugation::TooMuch               => Some(format!("{}{}", verb_stem(verb, VerbStem::Conjunctive, vt), "すぎる")),
        VerbConjugation::NonPastPolite         => Some(format!("{}{}", verb_stem(verb, VerbStem::Conjunctive, vt), "ます")),
        VerbConjugation::NonPastNegativePolite => Some(format!("{}{}", verb_stem(verb, VerbStem::Conjunctive, vt), "ません")),
        VerbConjugation::PastPolite            => Some(format!("{}{}", verb_stem(verb, VerbStem::Conjunctive, vt), "ました")),
        VerbConjugation::PastNegativePolite    => Some(format!("{}{}", verb_stem(verb, VerbStem::Conjunctive, vt), "ませんでした")),
        VerbConjugation::VolitionalPolite      => Some(format!("{}{}", verb_stem(verb, VerbStem::Conjunctive, vt), "ましょう")),
        VerbConjugation::HonorificI            => Some(format!("{}{}{}", "お", verb_stem(verb, VerbStem::Conjunctive, vt), "になる")),
        VerbConjugation::HonorificII           => Some(format!("{}{}{}", "お", verb_stem(verb, VerbStem::Conjunctive, vt), "なさる")),
        VerbConjugation::Humble                => Some(format!("{}{}{}", "お", verb_stem(verb, VerbStem::Conjunctive, vt), "する")),  

        // 未然形
        VerbConjugation::NonPastNegative  => Some(format!("{}{}", verb_stem(verb, VerbStem::Irrealis, vt), "ない")),
        VerbConjugation::PastNegative     => Some(format!("{}{}", verb_stem(verb, VerbStem::Irrealis, vt), "なかった")),
        
        VerbConjugation::Passive          =>
            match vt {
                VerbType::V1 => Some(format!("{}{}", verb_stem(verb, VerbStem::Irrealis, vt), "られる")),
                _  => Some(format!("{}{}", verb_stem(verb, VerbStem::Irrealis, vt), "れる"))
            },
        VerbConjugation::Causative        => 
            match vt {
                VerbType::V1 => Some(format!("{}{}", verb_stem(verb, VerbStem::Irrealis, vt), "させる")),
                _  => Some(format!("{}{}", verb_stem(verb, VerbStem::Irrealis, vt), "せる"))
            },
        VerbConjugation::CausativePassive =>
            match vt {
                VerbType::V1 => Some(format!("{}{}", verb_stem(verb, VerbStem::Irrealis, vt), "させられる")),
                _  => Some(format!("{}{}", verb_stem(verb, VerbStem::Irrealis, vt), "せられる"))
            },

        // 命令形

        VerbConjugation::Conditional =>
            match vt {
                VerbType::V1 => Some(format!("{}{}", verb_stem(verb, VerbStem::Imperative, vt), "れば")),
                _  => Some(format!("{}{}", verb_stem(verb, VerbStem::Imperative, vt), "ば"))
            },

        // 推量形
        VerbConjugation::Volitional   =>
            match vt {
                VerbType::V1 => Some(format!("{}{}", verb_stem(verb, VerbStem::Hypothetical, vt), "よう")),
                _  => Some(format!("{}{}", verb_stem(verb, VerbStem::Hypothetical, vt), "う"))
            },

        // テ形
        VerbConjugation::Try          => Some(format!("{}{}", verb_stem(verb, VerbStem::Te, vt), "みる")),

        // タ形
        VerbConjugation::Just         => Some(format!("{}{}", verb_stem(verb, VerbStem::Ta, vt), "ばかり")),
        
        // _ => None
    }
}

pub fn truncate_chars(s: &str, max_chars: usize) -> &str {
    match s.char_indices().nth(max_chars) {
        None => s,
        Some((idx, _)) => &s[..idx],
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn truncate_utf8() {
        let string = "いく".to_string().to_owned();
        let char_count = string.chars().count();
        let left_side = truncate_chars(&string, char_count - 1);
        assert_eq!(char_count, 2);
        assert_eq!(left_side, "い");
    }
    
    #[test]
    fn verb_type_from_str() {
        assert_eq!(VerbType::from_str("v1"), Ok(VerbType::V1));
        assert_ne!(VerbType::from_str("v1 "), Ok(VerbType::V1));
    }

    #[test]
    fn test_conjugate_verb() {
        assert_eq!(conjugate_verb("いく",VerbType::V5KS,VerbConjugation::NonPast), Some("いく".to_string()));
        assert_eq!(conjugate_verb("行く",VerbType::V5KS,VerbConjugation::Try), Some("行ってみる".to_string()));
        assert_eq!(conjugate_verb("いく",VerbType::V5KS,VerbConjugation::Try), Some("いってみる".to_string()));

        assert_eq!(conjugate_verb("行く",VerbType::V5KS,VerbConjugation::NonPastPolite),         Some("行きます".to_string()));
        assert_eq!(conjugate_verb("行く",VerbType::V5KS,VerbConjugation::NonPastNegativePolite), Some("行きません".to_string()));
        assert_eq!(conjugate_verb("行く",VerbType::V5KS,VerbConjugation::PastPolite),            Some("行きました".to_string()));
        assert_eq!(conjugate_verb("行く",VerbType::V5KS,VerbConjugation::PastNegativePolite),    Some("行きませんでした".to_string()));
        assert_eq!(conjugate_verb("行く",VerbType::V5KS,VerbConjugation::VolitionalPolite),      Some("行きましょう".to_string()));
        assert_eq!(conjugate_verb("行く",VerbType::V5KS,VerbConjugation::Conditional),           Some("行けば".to_string()));
        assert_eq!(conjugate_verb("行く",VerbType::V5KS,VerbConjugation::Volitional),            Some("行こう".to_string()));
        assert_eq!(conjugate_verb("行く",VerbType::V5KS,VerbConjugation::Just),                  Some("行ったばかり".to_string()));
        
        assert_eq!(conjugate_verb("食べる",VerbType::V1,VerbConjugation::Passive),          Some("食べられる".to_string()));
        assert_eq!(conjugate_verb("食べる",VerbType::V1,VerbConjugation::Causative),        Some("食べさせる".to_string()));
        assert_eq!(conjugate_verb("食べる",VerbType::V1,VerbConjugation::CausativePassive), Some("食べさせられる".to_string()));
        assert_eq!(conjugate_verb("食べる",VerbType::V1,VerbConjugation::Conditional), Some("食べれば".to_string()));
        assert_eq!(conjugate_verb("食べる",VerbType::V1,VerbConjugation::Volitional), Some("食べよう".to_string()));
        assert_eq!(conjugate_verb("食べる",VerbType::V1,VerbConjugation::Just), Some("食べたばかり".to_string()));
    }
}
