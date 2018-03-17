use okinawan::tables::*;
use util;

// if ん and next is ん or あいうえお　then n'
// if ん and is b or p then m

pub fn romaji_to_hiragana(romaji: &str) -> String {
    let romaji_len = romaji.len();

    let mut romaji_index = 0;
    let mut window = 1; // 1 to 3
    let mut hiragana = "".to_string();

    // convert 1 to 3 ascii chars to a single hirgana
    // i is starting point, window is length to be looked up
    while romaji_index < romaji_len && romaji_index + window < romaji_len + 1 {
        let romaji_sub = &romaji[romaji_index .. romaji_index + window];

        // check for gemminate consonant, if it exists, consume the char
        // add a "っ" and skip the lookup
        if window == 1 && romaji_index + 1 < romaji_len && util::is_consonant(romaji_sub) {
            let romaji_peek_ahead = &romaji[romaji_index + 1 .. romaji_index + 2];

            if romaji_sub == romaji_peek_ahead {
                hiragana = format!("{}{}", hiragana, "っ");
                romaji_index += 1;
                window = 1;
                continue;
            }
        }
        
        match util::lookup(romaji_sub, ROMAJI_TO_HIRAGANA_TABLE) {
            Some(hiragana_index) => {
                let single_hiragana = ROMAJI_TO_HIRAGANA_TABLE[hiragana_index].1.to_string();
                hiragana = format!("{}{}", hiragana, single_hiragana);
                romaji_index += window;
                window = 1;
            },
            None => {
                // romaji_sub was not found in the table, increase the window size if it was
                // not found. If the window is too large then move the index over and reset
                // the window size to 1
                if window < 3 {
                    window += 1;
                } else {
                    romaji_index += 1;
                    window = 1;
                }
            },
        }
    }
    
    hiragana.to_string()
}

pub fn hiragana_to_romaji(hiragana: &str) -> String {
    let hiragana_chars = hiragana.chars();
    let hiragana_len = hiragana_chars.count();

    let mut hiragana_index = 0;
    let mut window = 
        if hiragana_len > 1 {
            2
        } else {
            1
        };
    let mut romaji = "".to_string();

    while hiragana_index < hiragana_len && hiragana_index + window < hiragana_len + 1 {
        let hiragana_sub_vec = &hiragana.chars().collect::<Vec<_>>()[hiragana_index .. hiragana_index + window];
        let hiragana_sub_string: String = hiragana_sub_vec.into_iter().collect();
        let hiragana_sub: &str = &hiragana_sub_string[..];

        // look ahead
        if window == 1 && hiragana_index + 1 < hiragana_len {
            let hiragana_peek_ahead_vec = &hiragana.chars().collect::<Vec<_>>()[hiragana_index + 1 .. hiragana_index + 2];
            let hiragana_peek_ahead: String = hiragana_peek_ahead_vec.into_iter().collect();

            // "ん" as "n" or "m"
            if hiragana_sub == "ん" {
                let nasal = if util::hiragana_is_bilabial(&hiragana_peek_ahead) {
                    "m".to_string()
                } else if util::hiragana_merges_n(&hiragana_peek_ahead) {
                    "n'".to_string()
                } else {
                    "n".to_string()
                };
                
                romaji = format!("{}{}", romaji, nasal);
                hiragana_index += window;
                window =
                    if hiragana_len - hiragana_index > 1 {
                        2
                    } else {
                        1
                    };
                continue;
            }

            // check for gemminate consonant, if it exists, consume the char
            // add the geminnated consonant and skip the lookup
            if hiragana_sub == "っ" {
                let geminate = match util::lookup(&hiragana_peek_ahead, util::HIRAGANA_TO_GEMINATE_TABLE) {
                    Some(geminate_index) => util::HIRAGANA_TO_GEMINATE_TABLE[geminate_index].1.to_string(),
                    None => "".to_string()
                };
                romaji = format!("{}{}", romaji, geminate);
                hiragana_index += window;
                window =
                    if hiragana_len - hiragana_index > 1 {
                        2
                    } else {
                        1
                    };
                continue;
            }
        }

        if window == 2 {
            if hiragana_sub_vec[1] == 'ー' {
                match util::lookup(&hiragana_sub_vec[0].to_string(), util::HIRAGANA_TO_VOWEL_TABLE) {
                    Some(vowel_index) => {
                        match util::lookup(&hiragana_sub_vec[0].to_string(), HIRAGANA_TO_ROMAJI_TABLE) {
                            Some(romaji_index) => {
                                let prev = HIRAGANA_TO_ROMAJI_TABLE[romaji_index].1.to_string();
                                let hiragana_vowel = util::HIRAGANA_TO_VOWEL_TABLE[vowel_index].1.to_string();
                                match util::lookup(&hiragana_vowel, HIRAGANA_TO_ROMAJI_TABLE) {
                                    Some(romaji_vowel_index) => {
                                        let romaji_vowel = HIRAGANA_TO_ROMAJI_TABLE[romaji_vowel_index].1.to_string();
                                        romaji = format!("{}{}{}", romaji, prev, romaji_vowel);
                                        hiragana_index += window;
                                        window =
                                            if hiragana_len - hiragana_index > 1 {
                                                2
                                            } else {
                                                1
                                            };
                                        continue;
                                    },
                                    None => {
                                        ()
                                    }
                                }                                
                            },
                            None => {
                                ()
                            }
                        }
                    },
                    None => {
                        ()
                    }
                }
            }
        }

        match util::lookup(hiragana_sub, HIRAGANA_TO_ROMAJI_TABLE) {
            Some(romaji_index) => {
                let single_romaji = HIRAGANA_TO_ROMAJI_TABLE[romaji_index].1.to_string();
                romaji = format!("{}{}", romaji, single_romaji);
                hiragana_index += window;
                window =
                    if hiragana_len - hiragana_index > 1 {
                        2
                    } else {
                        1
                    };
            },
            None => {
                // hiragana_sub was not found in the table, increase the window size if it was
                // not found. If the window is too large then move the index over and reset
                // the window size relative to the length of string.
                if window > 1 {
                    window -= 1;
                } else {
                    hiragana_index += 1;
                    window =
                        if hiragana_len - hiragana_index > 1 {
                            2
                        } else {
                            1
                        };
                }
            },
        }
    }

    romaji.to_string()
}

pub fn replace_last_with_vowel(word: &str, vowel: &str) -> String {
    let word_len = word.chars().count();
    if word_len > 1 {
        let (left, right) = util::chars_split(word, word_len-2);

        match util::lookups_string(&right[..], vowel, HIRAGANA_TO_ROW_TABLE) {
            Some(tail) => {
                format!("{}{}", left, tail)
            },
            None       => {
                let (left, right) = util::chars_split(word, word_len-1);

                match util::lookups_string(&right[..], vowel, HIRAGANA_TO_ROW_TABLE) {
                    Some(tail) => format!("{}{}", left, tail),
                    None       => word.to_string()
                }
            }
        }
    } else if word_len == 1 {
        let (left, right) = util::chars_split(word, word_len-1);

        match util::lookups_string(&right[..], vowel, HIRAGANA_TO_ROW_TABLE) {
            Some(tail) => format!("{}{}", left, tail),
            None       => word.to_string()
        } 
    } else {
        word.to_string()
    }
}


pub fn remove_last_mora(word: &str) -> String {
    let word_len = word.chars().count();
    if word_len > 1 {
        let (left, right) = util::chars_split(word, word_len-2);
        if two_char_is_single_mora(&right) {
            format!("{}", left)            
        } else {
            let (left, _) = util::chars_split(word, word_len-1);
            format!("{}", left)
        }
    } else if word_len == 1 {
        let (left, _) = util::chars_split(word, word_len-1);
        format!("{}", left);
        word.to_string()
    } else {
        word.to_string()
    }
}

pub fn two_char_is_single_mora(s: &str) -> bool {
    s.chars().count() == 2 && ["いぃ","えぇ","きゃ","きゅ","きょ","ぎゃ","ぎゅ","ぎょ","くぃ","くぇ","くぉ","くゎ","ぐぃ","ぐぇ","ぐぉ","ぐゎ","しぇ","しゃ","しゅ","しょ","じぇ","じゃ","じゅ","じょ","すぃ","ずぃ","ちぇ","ちゃ","ちゅ","ちょ","ぢぃ","ぢぇ","ぢゃ","ぢゅ","ぢょ","っ","っや","っゆ","っよ","っわ","っゐ","っゑ","っを","っん","つ","つぁ","つぃ","つぇ","つぉ","てぃ","でぃ","とぅ","どぅ","にゃ","にゅ","にょ","ひゃ","ひゅ","ひょ","びゃ","びゅ","びょ","ぴゃ","ぴゅ","ぴょ","ふぁ","ふぃ","ふぇ","ふぉ","みゃ","みゅ","みょ","りゃ","りゅ","りょ","をぉ"].contains(&s)
}

/*

*/
