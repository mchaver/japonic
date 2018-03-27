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
くうじゅん　漕じゅん
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
