use util::{truncate_chars, split_chars_at, lookups_string};
use okinawan::tables::{HIRAGANA_TO_ROW_TABLE};
use okinawan::util::{remove_last_mora};
// use okinawan::util as uu;
// mod util;

pub enum VerbType {
    I1,
    I2,
    I3,
    I4,
    I5,
    I6,
    I7,
    I8,
    I9,
    I10,
    II1,
    II2,
    II3,
    II4,
    III,
    IV
}

/*
終止類
 断定非過去
 断定過去
 命令
 禁止
 意志
 推量
 強調
 疑問
 過去疑問

接続類
 連体非過去
 連体過去
 中止
 仮定
 理由

派生類
 否定
 丁寧
 使役
 受身
 可能
 尊敬
 継続
 希望
 のだ
*/


pub enum VerbStem {
    // 非過去否定
    Base,       // 基本語幹 base
    // 基本語幹+a  : N（否定）, riiN（可能・受身）, suN(使役), a/wa ば,　あ列
    // 基本語幹+ee : 条件形, 命令形　え列
    // 基本語幹+i  : 命令形　い列
    // 基本語幹+u  : na(な。禁止), ka(まで), kazirii(まで・かぎり)　う列

    
    Connective, // 連用語幹 connective
    // 連用語幹+i : ga(〜しに), ciroo(〜しそう), uusuN(〜できる), busaN(〜したい)　// い列
    // 連用語幹+(j)abiiN/ibiiN : // あ列　い列

    // 連用形 をり

    // 非過去
    Derivative, // 派生語幹 derivative stem　u/i/○
    // 派生語幹+uN/iN/N          : 終止形(現在形)
    // 派生語幹+uru/iru/ru       : baa(〜時)、hazi(〜はず), ru(ぞ)
    // 派生語幹+ura/ira/ra       : 疑問の助詞ga(か)
    // 派生語幹+uraa/iraa/raa    : 「〜なら」という条件を表す。
    // 派生語幹+u/i/○            : si（の）、siga（〜のだが）、sa（よ）、gutu（理由）、ga（疑問）、mi・i（たずね）
    // 派生語幹+utaN/itaN/taN    : 〜していた = 過去進行形
    // 派生語幹+uti/iti/ti       : 〜していたか = 過去進行中止形
    // 派生語幹+uteeN/iteeN/teeN : 〜していただろう = 過去進行推量形

    // 
    Euphonic    // 音便語幹 euphonic change stem　
    // 音便語幹+i   : 〜して　い列
    // 音便語幹+aN  : 〜した　あ列
    // 音便語幹+eeN : (今までに)きっと〜している,〜したに違いない,〜してある　え列
    // 音便語幹+ooN : 〜している　お列
}

pub enum VerbConjugation {
    NonPast,         // in/un/n 辞書形
    NonPastNegative, // ~an 否定形
    Past,            // ~an
    PastNegative,    // ~antan

    // ClauseEnding,    // i, does/ and 連用形
    // Connective,      // (y)a

    YesNoInterrogative, // ~mi
    WhInterrogative, // ~ga

    Honorific,
    Potential, // able to ~juusun
    Desiderative, // desire, want to
    Imperative, // ~ee
    Prohibitive, // prohibitive

    Volitional, // ~ra
    Causative, // ~sun
    Passive, // riiN rijuN
    Continuative, // ti form
    AttributiveNonPast, // N -> ru
    AttributivePast, // N -> ru
    Progressive, // ~oon, ~een
    
    Gerund,           // 音便語幹+i   : 〜して, ti ティ形
    NonPastPolite,   // biin
    NonPastNegativePolite, // biran
    InterrogativePolite, // biimi
    InterrogativePoliteII, // biiga
    PastPolite, // bitan
    PastNegativePoilte, // birantan
    InterrogativePastPolite, // bitii
    InterrogativePastPoliteII // bitiiga
}

fn remove_last_two_moras(s: &str) -> String {
    let first = remove_last_mora(s);
    remove_last_mora(&first)
}

pub fn base_stem(verb: &str, vt: VerbType) -> String {
    let bare_stem = remove_last_two_moras(verb);
    use self::VerbType::*;
    match vt {
        IV  => "".to_string(),
        III => format!("{}ら", remove_last_mora(verb)),
        II1 | II2 | II3 | II4  => format!("{}ら", bare_stem),
        I1 => format!("{}か", bare_stem),
        I2 => format!("{}が", bare_stem),
        I3 => format!("{}た", bare_stem),
        I4 => format!("{}た", bare_stem),
        I5 => format!("{}さ", bare_stem),
        I6 => format!("{}さ", bare_stem),
        I7 => format!("{}ば", bare_stem),
        I8 => format!("{}ま", bare_stem),
        I9 => format!("{}ら", bare_stem),
        I10 => format!("{}ら", bare_stem)
    }
}

pub fn connective_stem(verb: &str, vt: VerbType) -> String {
    let bare_stem = remove_last_two_moras(verb);
    use self::VerbType::*;
    match vt {
        IV  => "".to_string(),
        III => format!("{}あ", remove_last_mora(verb)),
        II1 | II2 | II3 | II4 => format!("{}あ", bare_stem),
        I1 => format!("{}ちゃ", bare_stem),
        I2 => format!("{}じゃ", bare_stem),
        I3 => format!("{}ちゃ", bare_stem),
        I4 => format!("{}ちゃ", bare_stem),
        I5 => format!("{}さ", bare_stem),
        I6 => format!("{}さ", bare_stem),
        I7 => format!("{}ば", bare_stem),
        I8 => format!("{}ま", bare_stem),
        I9 => format!("{}じゃ", bare_stem),
        I10 => format!("{}じゃ", bare_stem)
    }
}

pub fn derivative_stem(verb: &str, vt: VerbType) -> String {
    let bare_stem = remove_last_two_moras(verb);
    use self::VerbType::*;
    match vt {
        IV  => "".to_string(),
        III => remove_last_mora(verb),
        II1 | II2 | II3 | II4 => format!("{}あ", bare_stem),
        I1 => format!("{}ちゃ", bare_stem),
        I2 => format!("{}じゃ", bare_stem),
        I3 => format!("{}ちゃ", bare_stem),
        I4 => format!("{}ちゃ", bare_stem),
        I5 => format!("{}さ", bare_stem),
        I6 => format!("{}さ", bare_stem),
        I7 => format!("{}ば", bare_stem),
        I8 => format!("{}ま", bare_stem),
        I9 => format!("{}じゃ", bare_stem),
        I10 => format!("{}じゃ", bare_stem)
    }
}

pub fn euphonic_stem(verb: &str, vt: VerbType) -> String {
    let bare_stem = remove_last_two_moras(verb);
    use self::VerbType::*;
    match vt {
        IV  => "".to_string(),
        III => format!("{}た", remove_last_mora(verb)),
        II1 => format!("{}った", bare_stem),
        II2 => format!("{}た", bare_stem),
        II3 => format!("{}っちゃ", bare_stem),
        II4 => format!("{}ちゃ", bare_stem), // った
        I1 => format!("{}ちゃ", bare_stem),
        I2 => format!("{}じゃ", bare_stem),
        I3 => format!("{}っちゃ", bare_stem),
        I4 => format!("{}ちゃ", bare_stem),
        I5 => format!("{}ちゃ", bare_stem),
        I6 => format!("{}さ", bare_stem),
        I7 => format!("{}ら", bare_stem),
        I8 => format!("{}ら", bare_stem),
        I9 => format!("{}た", bare_stem),
        I10 => format!("{}ちゃ", bare_stem),
    }
}

pub fn replace_last_with_vowel(word: &str, vowel: &str) -> String {
    let word_len = word.chars().count();
    if word_len > 1 {
        let (left, right) = split_chars_at(word, word_len-2);

        match lookups_string(&right[..], vowel, HIRAGANA_TO_ROW_TABLE) {
            Some(tail) => {
                format!("{}{}", left, tail)
            },
            None       => {
                let (left, right) = split_chars_at(word, word_len-1);

                match lookups_string(&right[..], vowel, HIRAGANA_TO_ROW_TABLE) {
                    Some(tail) => format!("{}{}", left, tail),
                    None       => word.to_string()
                }
            }
        }
    } else if word_len == 1 {
        let (left, right) = split_chars_at(word, word_len-1);

        match lookups_string(&right[..], vowel, HIRAGANA_TO_ROW_TABLE) {
            Some(tail) => format!("{}{}", left, tail),
            None       => word.to_string()
        } 
    } else {
        word.to_string()
    }
}

pub fn conjugate_verb(verb: &str, vt: VerbType, conjugation: VerbConjugation) -> String {
    use self::VerbConjugation::*;
    use self::VerbType::*;
    match conjugation {
        // NonPast => verb.to_string(),
        AttributiveNonPast =>
            match vt {
                I1 | I2 | I3 | I4 | I5 | I6 | I7 | I8 | I9 | I10 => format!("{}{}", replace_last_with_vowel(&derivative_stem(verb, vt), "う"), "る"),
                II1 | II2 | II3 | II4 => format!("{}{}", replace_last_with_vowel(&derivative_stem(verb, vt), "い"), "る"),
                _ => format!("{}{}", replace_last_with_vowel(&derivative_stem(verb, vt), "う"), "る"), 
            },
        NonPast =>
            match vt {
                I1 | I2 | I3 | I4 | I5 | I6 | I7 | I8 | I9 | I10 => format!("{}{}", replace_last_with_vowel(&derivative_stem(verb, vt), "う"), "ん"),
                II1 | II2 | II3 | II4 => format!("{}{}", replace_last_with_vowel(&derivative_stem(verb, vt), "い"), "ん"),
                III => verb.to_string(),
                _ => format!("{}{}", replace_last_with_vowel(&derivative_stem(verb, vt), "う"), "ん"), 
            },
        NonPastNegative => format!("{}{}", replace_last_with_vowel(&base_stem(verb, vt), "あ"), "ん"),
        PastNegative => format!("{}{}", replace_last_with_vowel(&base_stem(verb, vt), "あ"), "んたん"),
        Past => format!("{}{}", replace_last_with_vowel(&euphonic_stem(verb, vt), "あ"), "ん"),
        AttributivePast => format!("{}{}", replace_last_with_vowel(&euphonic_stem(verb, vt), "あ"), "る"),
        NonPastPolite => match vt {
            II1 | II2 | II3 | II4 => format!("{}{}", replace_last_with_vowel(&connective_stem(verb, vt), "い"), "びーん"), // やびーん
            III => format!("{}{}", replace_last_with_vowel(&connective_stem(verb, vt), "い"), "びーん"),
            _ => format!("{}{}", replace_last_with_vowel(&connective_stem(verb, vt), "あ"), "びーん"),

        }

        YesNoInterrogative =>
            match vt {
                II1 | II2 | II3 | II4 => format!("{}{}", replace_last_with_vowel(&derivative_stem(verb, vt), "い"), "み"),
                _ => format!("{}{}", replace_last_with_vowel(&derivative_stem(verb, vt), "う"), "み"),
            },
        WhInterrogative => 
            match vt {
                II1 | II2 | II3 | II4 => format!("{}{}", replace_last_with_vowel(&derivative_stem(verb, vt), "い"), "が"),
                _ => format!("{}{}", replace_last_with_vowel(&derivative_stem(verb, vt), "う"), "が"),
            },
        Honorific =>
            match vt {
                // II1 | II2 | II3 | II4 => format!("{}{}", connective_stem(verb, vt), "みせーん"),
                _ => format!("{}{}", replace_last_with_vowel(&connective_stem(verb, vt), "い"), "みせーん"),
            },
        Potential => format!("{}{}", replace_last_with_vowel(&connective_stem(verb, vt), "い"), "ゆーすん"),
        Desiderative => format!("{}{}", replace_last_with_vowel(&connective_stem(verb, vt), "い"), "ぶさん"),
        Imperative => format!("{}{}", replace_last_with_vowel(&base_stem(verb, vt), "え"), "ー"),

        Prohibitive =>
            match vt {
                // II1 | II2 | II3 | II4 => format!("{}{}", base_stem(verb, vt), "んな"), // んな
                _ => format!("{}{}", replace_last_with_vowel(&base_stem(verb, vt), "う"), "な"),
            },

        Gerund | Continuative => replace_last_with_vowel(&euphonic_stem(verb, vt), "い"),
        Progressive => format!("{}{}", replace_last_with_vowel(&euphonic_stem(verb, vt), "お"), "ーん"), // えーん
            
        Causative => format!("{}{}", &base_stem(verb, vt), "すん"),
        Passive => format!("{}{}", &base_stem(verb, vt), "りゆん"), // りーん

        Volitional => replace_last_with_vowel(&base_stem(verb, vt), "あ"), // let's

        _ => verb.to_string()
    }
}
