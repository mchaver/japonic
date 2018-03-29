use okinawan::util::{two_char_is_single_mora, remove_last_mora};
use okinawan::lib::*;

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_replace_last_with_vowel() {
        assert_eq!(replace_last_with_vowel("かた","あ"), "かた".to_string());
        assert_eq!(replace_last_with_vowel("かた","い"), "かてぃ".to_string());
        assert_eq!(replace_last_with_vowel("かた","う"), "かとぅ".to_string());
        assert_eq!(replace_last_with_vowel("かた","え"), "かて".to_string());
        assert_eq!(replace_last_with_vowel("かた","お"), "かと".to_string());

        assert_eq!(replace_last_with_vowel("かちゃ","あ"), "かちゃ".to_string());
        assert_eq!(replace_last_with_vowel("かちゃ","い"), "かち".to_string());
        assert_eq!(replace_last_with_vowel("かちゃ","う"), "かちゅ".to_string());
        assert_eq!(replace_last_with_vowel("かちゃ","え"), "かちぇ".to_string());
        assert_eq!(replace_last_with_vowel("かちゃ","お"), "かちょ".to_string());

        assert_eq!(replace_last_with_vowel("ああ","あ"), "ああ".to_string());
        assert_eq!(replace_last_with_vowel("ああ","い"), "あい".to_string());
        assert_eq!(replace_last_with_vowel("ああ","う"), "あう".to_string());
        assert_eq!(replace_last_with_vowel("ああ","え"), "あえ".to_string());
        assert_eq!(replace_last_with_vowel("ああ","お"), "あお".to_string());
        
        assert_eq!(replace_last_with_vowel("あ","あ"), "あ".to_string());
        assert_eq!(replace_last_with_vowel("あ","い"), "い".to_string());
        assert_eq!(replace_last_with_vowel("あ","う"), "う".to_string());
        assert_eq!(replace_last_with_vowel("あ","え"), "え".to_string());
        assert_eq!(replace_last_with_vowel("あ","お"), "お".to_string());
    }

    #[test]
    fn test_two_char_is_single_mora() {
        assert!(two_char_is_single_mora("っん"));
        assert!(two_char_is_single_mora("っゑ"));
        assert!(two_char_is_single_mora("っわ"));
        assert!(two_char_is_single_mora("びょ"));
        assert!(two_char_is_single_mora("ちゅ"));
        
        assert!(!two_char_is_single_mora("ん"));
        assert!(!two_char_is_single_mora("わ"));
        assert!(!two_char_is_single_mora("び"));
    }

    #[test]
    fn test_remove_last_mora() {
        assert_eq!(remove_last_mora("かちゃ"), "か");
        assert_eq!(remove_last_mora("かち"), "か");
        assert_eq!(remove_last_mora("かちゅ"), "か");
        assert_eq!(remove_last_mora("かちぇ"), "か");
        assert_eq!(remove_last_mora("かちょ"), "か");

        assert_eq!(remove_last_mora("まま"), "ま");
        assert_eq!(remove_last_mora("まみ"), "ま");
        assert_eq!(remove_last_mora("まむ"), "ま");
        assert_eq!(remove_last_mora("まめ"), "ま");
        assert_eq!(remove_last_mora("まも"), "ま"); 
    }

    #[test]
    fn test__stems() {
        assert_eq!(base_stem("かちゅん",VerbType::I1), "かか".to_string());
        assert_eq!(connective_stem("かちゅん",VerbType::I1), "かちゃ".to_string());
        assert_eq!(derivative_stem("かちゅん",VerbType::I1), "かちゃ".to_string());
        assert_eq!(euphonic_stem("かちゅん",VerbType::I1), "かちゃ".to_string());
    }

    #[test]
    fn test_conjugate_verb() {
        assert_eq!(conjugate_verb("かちゅん", VerbType::I1, VerbConjugation::NonPast ), "かちゅん".to_string());
        assert_eq!(conjugate_verb("かちゅん", VerbType::I1, VerbConjugation::NonPastNegative ), "かかん".to_string());
        assert_eq!(conjugate_verb("かちゅん", VerbType::I1, VerbConjugation::PastNegative ), "かかんたん".to_string());
        assert_eq!(conjugate_verb("かちゅん", VerbType::I1, VerbConjugation::Past ), "かちゃん".to_string());
        assert_eq!(conjugate_verb("かちゅん", VerbType::I1, VerbConjugation::NonPastPolite ), "かちゃびーん".to_string());
        assert_eq!(conjugate_verb("かちゅん", VerbType::I1, VerbConjugation::YesNoInterrogative ), "かちゅみ".to_string());
        assert_eq!(conjugate_verb("かちゅん", VerbType::I1, VerbConjugation::WhInterrogative ), "かちゅが".to_string());
        assert_eq!(conjugate_verb("かちゅん", VerbType::I1, VerbConjugation::Honorific ), "かちみせーん".to_string());
        assert_eq!(conjugate_verb("かちゅん", VerbType::I1, VerbConjugation::Imperative ), "かけー".to_string());
        assert_eq!(conjugate_verb("かちゅん", VerbType::I1, VerbConjugation::Prohibitive ), "かくな".to_string());
        assert_eq!(conjugate_verb("かちゅん", VerbType::I1, VerbConjugation::Desiderative ), "かちぶさん".to_string());

        assert_eq!(conjugate_verb("くーじゅん", VerbType::I2, VerbConjugation::NonPast ), "くーじゅん".to_string());
        assert_eq!(conjugate_verb("くーじゅん", VerbType::I2, VerbConjugation::NonPastNegative ), "くーがん".to_string());
        assert_eq!(conjugate_verb("くーじゅん", VerbType::I2, VerbConjugation::PastNegative ), "くーがんたん".to_string());
        assert_eq!(conjugate_verb("くーじゅん", VerbType::I2, VerbConjugation::Past ), "くーじゃん".to_string());
        assert_eq!(conjugate_verb("くーじゅん", VerbType::I2, VerbConjugation::NonPastPolite ), "くーじゃびーん".to_string());
        assert_eq!(conjugate_verb("くーじゅん", VerbType::I2, VerbConjugation::YesNoInterrogative ), "くーじゅみ".to_string());
        assert_eq!(conjugate_verb("くーじゅん", VerbType::I2, VerbConjugation::WhInterrogative ), "くーじゅが".to_string());
        assert_eq!(conjugate_verb("くーじゅん", VerbType::I2, VerbConjugation::Honorific ), "くーじみせーん".to_string());
        assert_eq!(conjugate_verb("くーじゅん", VerbType::I2, VerbConjugation::Imperative ), "くーげー".to_string());
        assert_eq!(conjugate_verb("くーじゅん", VerbType::I2, VerbConjugation::Prohibitive ), "くーぐな".to_string());
        assert_eq!(conjugate_verb("くーじゅん", VerbType::I2, VerbConjugation::Desiderative ), "くーじぶさん".to_string());
                
        assert_eq!(conjugate_verb("むちゅん", VerbType::I3, VerbConjugation::NonPast ), "むちゅん".to_string());
        assert_eq!(conjugate_verb("むちゅん", VerbType::I3, VerbConjugation::NonPastNegative ), "むたん".to_string());
        assert_eq!(conjugate_verb("むちゅん", VerbType::I3, VerbConjugation::PastNegative ), "むたんたん".to_string());
        assert_eq!(conjugate_verb("むちゅん", VerbType::I3, VerbConjugation::Past ), "むっちゃん".to_string());
        assert_eq!(conjugate_verb("むちゅん", VerbType::I3, VerbConjugation::NonPastPolite ), "むちゃびーん".to_string());
        assert_eq!(conjugate_verb("むちゅん", VerbType::I3, VerbConjugation::YesNoInterrogative ), "むちゅみ".to_string());
        assert_eq!(conjugate_verb("むちゅん", VerbType::I3, VerbConjugation::WhInterrogative ), "むちゅが".to_string());
        assert_eq!(conjugate_verb("むちゅん", VerbType::I3, VerbConjugation::Honorific ), "むちみせーん".to_string());
        assert_eq!(conjugate_verb("むちゅん", VerbType::I3, VerbConjugation::Imperative ), "むてー".to_string());
        assert_eq!(conjugate_verb("むちゅん", VerbType::I3, VerbConjugation::Prohibitive ), "むとぅな".to_string());
        assert_eq!(conjugate_verb("むちゅん", VerbType::I3, VerbConjugation::Desiderative ), "むちぶさん".to_string());
        assert_eq!(conjugate_verb("むちゅん", VerbType::I3, VerbConjugation::AttributiveNonPast ), "むちゅる".to_string());
        assert_eq!(conjugate_verb("むちゅん", VerbType::I3, VerbConjugation::Progressive ), "むっちょーん".to_string());
        assert_eq!(conjugate_verb("むちゅん", VerbType::I3, VerbConjugation::Continuative ), "むっち".to_string());
        assert_eq!(conjugate_verb("むちゅん", VerbType::I3, VerbConjugation::Causative ), "むたすん".to_string());
        assert_eq!(conjugate_verb("むちゅん", VerbType::I3, VerbConjugation::Passive ), "むたりゆん".to_string());

        assert_eq!(conjugate_verb("すらちゅん", VerbType::I4, VerbConjugation::NonPast ), "すらちゅん".to_string());
        assert_eq!(conjugate_verb("すらちゅん", VerbType::I4, VerbConjugation::NonPastNegative ), "すらたん".to_string());
        assert_eq!(conjugate_verb("すらちゅん", VerbType::I4, VerbConjugation::PastNegative ), "すらたんたん".to_string());
        assert_eq!(conjugate_verb("すらちゅん", VerbType::I4, VerbConjugation::Past ), "すらちゃん".to_string());
        assert_eq!(conjugate_verb("すらちゅん", VerbType::I4, VerbConjugation::NonPastPolite ), "すらちゃびーん".to_string());
        assert_eq!(conjugate_verb("すらちゅん", VerbType::I4, VerbConjugation::YesNoInterrogative ), "すらちゅみ".to_string());
        assert_eq!(conjugate_verb("すらちゅん", VerbType::I4, VerbConjugation::WhInterrogative ), "すらちゅが".to_string());
        assert_eq!(conjugate_verb("すらちゅん", VerbType::I4, VerbConjugation::Honorific ), "すらちみせーん".to_string());
        assert_eq!(conjugate_verb("すらちゅん", VerbType::I4, VerbConjugation::Imperative ), "すらてー".to_string());
        assert_eq!(conjugate_verb("すらちゅん", VerbType::I4, VerbConjugation::Prohibitive ), "すらとぅな".to_string());
        assert_eq!(conjugate_verb("すらちゅん", VerbType::I4, VerbConjugation::Desiderative ), "すらちぶさん".to_string());
        assert_eq!(conjugate_verb("すらちゅん", VerbType::I4, VerbConjugation::AttributiveNonPast ), "すらちゅる".to_string());
        assert_eq!(conjugate_verb("すらちゅん", VerbType::I4, VerbConjugation::Progressive ), "すらちょーん".to_string());
        assert_eq!(conjugate_verb("すらちゅん", VerbType::I4, VerbConjugation::Continuative ), "すらち".to_string());
        assert_eq!(conjugate_verb("すらちゅん", VerbType::I4, VerbConjugation::Causative ), "すらたすん".to_string());
        assert_eq!(conjugate_verb("すらちゅん", VerbType::I4, VerbConjugation::Passive ), "すらたりゆん".to_string());

        assert_eq!(conjugate_verb("くるすん", VerbType::I5, VerbConjugation::NonPast ), "くるすん".to_string());
        assert_eq!(conjugate_verb("くるすん", VerbType::I5, VerbConjugation::NonPastNegative ), "くるさん".to_string());
        assert_eq!(conjugate_verb("くるすん", VerbType::I5, VerbConjugation::PastNegative ), "くるさんたん".to_string());
        assert_eq!(conjugate_verb("くるすん", VerbType::I5, VerbConjugation::Past ), "くるちゃん".to_string());
        assert_eq!(conjugate_verb("くるすん", VerbType::I5, VerbConjugation::NonPastPolite ), "くるさびーん".to_string());
        assert_eq!(conjugate_verb("くるすん", VerbType::I5, VerbConjugation::YesNoInterrogative ), "くるすみ".to_string());
        assert_eq!(conjugate_verb("くるすん", VerbType::I5, VerbConjugation::WhInterrogative ), "くるすが".to_string());
        assert_eq!(conjugate_verb("くるすん", VerbType::I5, VerbConjugation::Honorific ), "くるすぃみせーん".to_string());
        assert_eq!(conjugate_verb("くるすん", VerbType::I5, VerbConjugation::Imperative ), "くるせー".to_string());
        assert_eq!(conjugate_verb("くるすん", VerbType::I5, VerbConjugation::Prohibitive ), "くるすな".to_string());
        assert_eq!(conjugate_verb("くるすん", VerbType::I5, VerbConjugation::Desiderative ), "くるすぃぶさん".to_string());
        assert_eq!(conjugate_verb("くるすん", VerbType::I5, VerbConjugation::AttributiveNonPast ), "くるする".to_string());
        assert_eq!(conjugate_verb("くるすん", VerbType::I5, VerbConjugation::Progressive ), "くるちょーん".to_string());
        assert_eq!(conjugate_verb("くるすん", VerbType::I5, VerbConjugation::Continuative ), "くるち".to_string());
        assert_eq!(conjugate_verb("くるすん", VerbType::I5, VerbConjugation::Causative ), "くるさすん".to_string());
        assert_eq!(conjugate_verb("くるすん", VerbType::I5, VerbConjugation::Passive ), "くるさりゆん".to_string());

        assert_eq!(conjugate_verb("ひっこーすん", VerbType::I6, VerbConjugation::NonPast ), "ひっこーすん".to_string());
        assert_eq!(conjugate_verb("ひっこーすん", VerbType::I6, VerbConjugation::NonPastNegative ), "ひっこーさん".to_string());
        assert_eq!(conjugate_verb("ひっこーすん", VerbType::I6, VerbConjugation::PastNegative ), "ひっこーさんたん".to_string());
        assert_eq!(conjugate_verb("ひっこーすん", VerbType::I6, VerbConjugation::Past ), "ひっこーさん".to_string());
        assert_eq!(conjugate_verb("ひっこーすん", VerbType::I6, VerbConjugation::NonPastPolite ), "ひっこーさびーん".to_string());
        assert_eq!(conjugate_verb("ひっこーすん", VerbType::I6, VerbConjugation::YesNoInterrogative ), "ひっこーすみ".to_string());
        assert_eq!(conjugate_verb("ひっこーすん", VerbType::I6, VerbConjugation::WhInterrogative ), "ひっこーすが".to_string());
        assert_eq!(conjugate_verb("ひっこーすん", VerbType::I6, VerbConjugation::Honorific ), "ひっこーすぃみせーん".to_string());
        assert_eq!(conjugate_verb("ひっこーすん", VerbType::I6, VerbConjugation::Imperative ), "ひっこーせー".to_string());
        assert_eq!(conjugate_verb("ひっこーすん", VerbType::I6, VerbConjugation::Prohibitive ), "ひっこーすな".to_string());
        assert_eq!(conjugate_verb("ひっこーすん", VerbType::I6, VerbConjugation::Desiderative ), "ひっこーすぃぶさん".to_string());
        assert_eq!(conjugate_verb("ひっこーすん", VerbType::I6, VerbConjugation::AttributiveNonPast ), "ひっこーする".to_string());
        assert_eq!(conjugate_verb("ひっこーすん", VerbType::I6, VerbConjugation::Progressive ), "ひっこーそーん".to_string());
        assert_eq!(conjugate_verb("ひっこーすん", VerbType::I6, VerbConjugation::Continuative ), "ひっこーすぃ".to_string());
        assert_eq!(conjugate_verb("ひっこーすん", VerbType::I6, VerbConjugation::Causative ), "ひっこーさすん".to_string());
        assert_eq!(conjugate_verb("ひっこーすん", VerbType::I6, VerbConjugation::Passive ), "ひっこーさりゆん".to_string());

        assert_eq!(conjugate_verb("ゆぶん", VerbType::I7, VerbConjugation::NonPast ), "ゆぶん".to_string());
        assert_eq!(conjugate_verb("ゆぶん", VerbType::I7, VerbConjugation::NonPastNegative ), "ゆばん".to_string());
        assert_eq!(conjugate_verb("ゆぶん", VerbType::I7, VerbConjugation::PastNegative ), "ゆばんたん".to_string());
        assert_eq!(conjugate_verb("ゆぶん", VerbType::I7, VerbConjugation::Past ), "ゆらん".to_string());
        assert_eq!(conjugate_verb("ゆぶん", VerbType::I7, VerbConjugation::NonPastPolite ), "ゆばびーん".to_string());
        assert_eq!(conjugate_verb("ゆぶん", VerbType::I7, VerbConjugation::YesNoInterrogative ), "ゆぶみ".to_string());
        assert_eq!(conjugate_verb("ゆぶん", VerbType::I7, VerbConjugation::WhInterrogative ), "ゆぶが".to_string());
        assert_eq!(conjugate_verb("ゆぶん", VerbType::I7, VerbConjugation::Honorific ), "ゆびみせーん".to_string());
        assert_eq!(conjugate_verb("ゆぶん", VerbType::I7, VerbConjugation::Imperative ), "ゆべー".to_string());
        assert_eq!(conjugate_verb("ゆぶん", VerbType::I7, VerbConjugation::Prohibitive ), "ゆぶな".to_string());
        assert_eq!(conjugate_verb("ゆぶん", VerbType::I7, VerbConjugation::Desiderative ), "ゆびぶさん".to_string());
        assert_eq!(conjugate_verb("ゆぶん", VerbType::I7, VerbConjugation::AttributiveNonPast ), "ゆぶる".to_string());
        assert_eq!(conjugate_verb("ゆぶん", VerbType::I7, VerbConjugation::Progressive ), "ゆろーん".to_string());
        assert_eq!(conjugate_verb("ゆぶん", VerbType::I7, VerbConjugation::Continuative ), "ゆり".to_string());
        assert_eq!(conjugate_verb("ゆぶん", VerbType::I7, VerbConjugation::Causative ), "ゆばすん".to_string());
        assert_eq!(conjugate_verb("ゆぶん", VerbType::I7, VerbConjugation::Passive ), "ゆばりゆん".to_string());
        
        assert_eq!(conjugate_verb("ゆむん", VerbType::I8, VerbConjugation::NonPast ), "ゆむん".to_string());
        assert_eq!(conjugate_verb("ゆむん", VerbType::I8, VerbConjugation::NonPastNegative ), "ゆまん".to_string());
        assert_eq!(conjugate_verb("ゆむん", VerbType::I8, VerbConjugation::PastNegative ), "ゆまんたん".to_string());
        assert_eq!(conjugate_verb("ゆむん", VerbType::I8, VerbConjugation::Past ), "ゆらん".to_string());
        assert_eq!(conjugate_verb("ゆむん", VerbType::I8, VerbConjugation::NonPastPolite ), "ゆまびーん".to_string());
        assert_eq!(conjugate_verb("ゆむん", VerbType::I8, VerbConjugation::YesNoInterrogative ), "ゆむみ".to_string());
        assert_eq!(conjugate_verb("ゆむん", VerbType::I8, VerbConjugation::WhInterrogative ), "ゆむが".to_string());
        assert_eq!(conjugate_verb("ゆむん", VerbType::I8, VerbConjugation::Honorific ), "ゆみみせーん".to_string());
        assert_eq!(conjugate_verb("ゆむん", VerbType::I8, VerbConjugation::Imperative ), "ゆめー".to_string());
        assert_eq!(conjugate_verb("ゆむん", VerbType::I8, VerbConjugation::Prohibitive ), "ゆむな".to_string());
        assert_eq!(conjugate_verb("ゆむん", VerbType::I8, VerbConjugation::Desiderative ), "ゆみぶさん".to_string());
        assert_eq!(conjugate_verb("ゆむん", VerbType::I8, VerbConjugation::AttributiveNonPast ), "ゆむる".to_string());
        assert_eq!(conjugate_verb("ゆむん", VerbType::I8, VerbConjugation::Progressive ), "ゆろーん".to_string());
        assert_eq!(conjugate_verb("ゆむん", VerbType::I8, VerbConjugation::Continuative ), "ゆり".to_string());
        assert_eq!(conjugate_verb("ゆむん", VerbType::I8, VerbConjugation::Causative ), "ゆますん".to_string());
        assert_eq!(conjugate_verb("ゆむん", VerbType::I8, VerbConjugation::Passive ), "ゆまりゆん".to_string());

        assert_eq!(conjugate_verb("にんじゅん", VerbType::I9, VerbConjugation::NonPast ), "にんじゅん".to_string());
        assert_eq!(conjugate_verb("にんじゅん", VerbType::I9, VerbConjugation::NonPastNegative ), "にんらん".to_string());
        assert_eq!(conjugate_verb("にんじゅん", VerbType::I9, VerbConjugation::PastNegative ), "にんらんたん".to_string());
        assert_eq!(conjugate_verb("にんじゅん", VerbType::I9, VerbConjugation::Past ), "にんたん".to_string());
        assert_eq!(conjugate_verb("にんじゅん", VerbType::I9, VerbConjugation::NonPastPolite ), "にんじゃびーん".to_string());
        assert_eq!(conjugate_verb("にんじゅん", VerbType::I9, VerbConjugation::YesNoInterrogative ), "にんじゅみ".to_string());
        assert_eq!(conjugate_verb("にんじゅん", VerbType::I9, VerbConjugation::WhInterrogative ), "にんじゅが".to_string());
        assert_eq!(conjugate_verb("にんじゅん", VerbType::I9, VerbConjugation::Honorific ), "にんじみせーん".to_string());
        assert_eq!(conjugate_verb("にんじゅん", VerbType::I9, VerbConjugation::Imperative ), "にんれー".to_string());
        assert_eq!(conjugate_verb("にんじゅん", VerbType::I9, VerbConjugation::Prohibitive ), "にんるな".to_string());
        assert_eq!(conjugate_verb("にんじゅん", VerbType::I9, VerbConjugation::Desiderative ), "にんじぶさん".to_string());
        assert_eq!(conjugate_verb("にんじゅん", VerbType::I9, VerbConjugation::AttributiveNonPast ), "にんじゅる".to_string());
        assert_eq!(conjugate_verb("にんじゅん", VerbType::I9, VerbConjugation::Progressive ), "にんとーん".to_string());
        assert_eq!(conjugate_verb("にんじゅん", VerbType::I9, VerbConjugation::Continuative ), "にんてぃ".to_string());
        assert_eq!(conjugate_verb("にんじゅん", VerbType::I9, VerbConjugation::Causative ), "にんらすん".to_string());
        assert_eq!(conjugate_verb("にんじゅん", VerbType::I9, VerbConjugation::Passive ), "にんらりゆん".to_string());

        assert_eq!(conjugate_verb("んんじゅん", VerbType::I10, VerbConjugation::NonPast ), "んんじゅん".to_string());
        assert_eq!(conjugate_verb("んんじゅん", VerbType::I10, VerbConjugation::NonPastNegative ), "んんらん".to_string());
        assert_eq!(conjugate_verb("んんじゅん", VerbType::I10, VerbConjugation::PastNegative ), "んんらんたん".to_string());
        assert_eq!(conjugate_verb("んんじゅん", VerbType::I10, VerbConjugation::Past ), "んんちゃん".to_string());
        assert_eq!(conjugate_verb("んんじゅん", VerbType::I10, VerbConjugation::NonPastPolite ), "んんじゃびーん".to_string());
        assert_eq!(conjugate_verb("んんじゅん", VerbType::I10, VerbConjugation::YesNoInterrogative ), "んんじゅみ".to_string());
        assert_eq!(conjugate_verb("んんじゅん", VerbType::I10, VerbConjugation::WhInterrogative ), "んんじゅが".to_string());
        assert_eq!(conjugate_verb("んんじゅん", VerbType::I10, VerbConjugation::Honorific ), "んんじみせーん".to_string());
        assert_eq!(conjugate_verb("んんじゅん", VerbType::I10, VerbConjugation::Imperative ), "んんれー".to_string());
        assert_eq!(conjugate_verb("んんじゅん", VerbType::I10, VerbConjugation::Prohibitive ), "んんるな".to_string());
        assert_eq!(conjugate_verb("んんじゅん", VerbType::I10, VerbConjugation::Desiderative ), "んんじぶさん".to_string());
        assert_eq!(conjugate_verb("んんじゅん", VerbType::I10, VerbConjugation::AttributiveNonPast ), "んんじゅる".to_string());
        assert_eq!(conjugate_verb("んんじゅん", VerbType::I10, VerbConjugation::Progressive ), "んんちょーん".to_string());
        assert_eq!(conjugate_verb("んんじゅん", VerbType::I10, VerbConjugation::Continuative ), "んんち".to_string());
        assert_eq!(conjugate_verb("んんじゅん", VerbType::I10, VerbConjugation::Causative ), "んんらすん".to_string());
        assert_eq!(conjugate_verb("んんじゅん", VerbType::I10, VerbConjugation::Passive ), "んんらりゆん".to_string());
        
        assert_eq!(conjugate_verb("とぅいん", VerbType::II2, VerbConjugation::NonPast ), "とぅいん".to_string());
        assert_eq!(conjugate_verb("とぅいん", VerbType::II2, VerbConjugation::NonPastNegative ), "とぅらん".to_string());
        assert_eq!(conjugate_verb("とぅいん", VerbType::II2, VerbConjugation::PastNegative ), "とぅらんたん".to_string());
        assert_eq!(conjugate_verb("とぅいん", VerbType::II2, VerbConjugation::Past ), "とぅたん".to_string()); // ら
        assert_eq!(conjugate_verb("とぅいん", VerbType::II2, VerbConjugation::NonPastPolite ), "とぅいびーん".to_string());
        assert_eq!(conjugate_verb("とぅいん", VerbType::II2, VerbConjugation::YesNoInterrogative ), "とぅいみ".to_string());
        assert_eq!(conjugate_verb("とぅいん", VerbType::II2, VerbConjugation::WhInterrogative ), "とぅいが".to_string());
        assert_eq!(conjugate_verb("とぅいん", VerbType::II2, VerbConjugation::Honorific ), "とぅいみせーん".to_string());
        assert_eq!(conjugate_verb("とぅいん", VerbType::II2, VerbConjugation::Imperative ), "とぅれー".to_string());
        assert_eq!(conjugate_verb("とぅいん", VerbType::II2, VerbConjugation::Prohibitive ), "とぅるな".to_string()); // とぅんな
        assert_eq!(conjugate_verb("とぅいん", VerbType::II2, VerbConjugation::Desiderative ), "とぅいぶさん".to_string());

        assert_eq!(conjugate_verb("とぅいん", VerbType::II2, VerbConjugation::AttributiveNonPast ), "とぅいる".to_string());
        assert_eq!(conjugate_verb("とぅいん", VerbType::II2, VerbConjugation::Progressive ), "とぅとーん".to_string()); // とぅろーん
        assert_eq!(conjugate_verb("とぅいん", VerbType::II2, VerbConjugation::Continuative ), "とぅてぃ".to_string());
        assert_eq!(conjugate_verb("とぅいん", VerbType::II2, VerbConjugation::Causative ), "とぅらすん".to_string());
        assert_eq!(conjugate_verb("とぅいん", VerbType::II2, VerbConjugation::Passive ), "とぅらりゆん".to_string());
        
    }
}

/*
I1
かちゅん　書ちゅん
ちちゅん　聞ちゅん
さちゅん　咲ちゅん
っあっちゅん　歩ちゅん

I2
くーじゅん　漕じゅん
っゐーじゅん　泳じゅん
っおーじゅん　扇じゅん

I3
たちゅん　立ちゅん
っうちゅん　打ちゅん
かちゅん　勝ちゅん

I4
すらちゅん　育ちゅん
たむちゅん　保ちゅん
くちゅん　朽ちゅん

I5
くるすん　殺すん
めーすん　燃すん
はんすん　外すん

I6
すん　為ん
しっくぁすん　敷くぁすん
ひっこーすん　比較すん

I7
ゆぶん　呼ぶん
とぅぶん　飛ぶん
むすぶん　結ぶん

I8
ゆむん　読むん
ぬむん　飲むん
っあむん　編むん

I9
にんじゅん　眠じゅん
かんじゅん　被じゅん
っあんじゅん

I10
んんじゅん　見じゅん
くんじゅん　括じゅん

II1
とぅいん　取いん

II2
かいん　刈いん
ぬぶいん　登いん
っあらいん　洗いん

II3
きーん　蹴ーん
っいーん　入ーん
ひーん　放ーん
ちーん　切ーん

II4
にーん　煮ーん
ちーん　着ーん
っいーん　言ーん
いーん　座ーん

III
あん　有ん
うん　居ん
やん　
*/
