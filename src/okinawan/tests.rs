use okinawan::util::*;

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
    }
}
