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
        assert_eq!(conjugate_verb("とぅいん", VerbType::II2, VerbConjugation::Past ), "とぅらん".to_string()); // た
        assert_eq!(conjugate_verb("とぅいん", VerbType::II2, VerbConjugation::NonPastPolite ), "とぅいびーん".to_string());
        assert_eq!(conjugate_verb("とぅいん", VerbType::II2, VerbConjugation::YesNoInterrogative ), "とぅいみ".to_string());
        assert_eq!(conjugate_verb("とぅいん", VerbType::II2, VerbConjugation::WhInterrogative ), "とぅいが".to_string());
        assert_eq!(conjugate_verb("とぅいん", VerbType::II2, VerbConjugation::Honorific ), "とぅいみせーん".to_string());
        assert_eq!(conjugate_verb("とぅいん", VerbType::II2, VerbConjugation::Imperative ), "とぅれー".to_string());
        assert_eq!(conjugate_verb("とぅいん", VerbType::II2, VerbConjugation::Prohibitive ), "とぅるな".to_string()); // とぅんな
        assert_eq!(conjugate_verb("とぅいん", VerbType::II2, VerbConjugation::Desiderative ), "とぅいぶさん".to_string());


    }
}
