use standard::tables::*;
use util;

extern crate unicode_segmentation;

use self::unicode_segmentation::UnicodeSegmentation;

pub fn is_romaji(s: &str) -> bool {
    s.len() == 1 && ["a","b","c","d","e","f","g","h","i","j","k","l","m","n","o","p","q","r","s","t","u","v","w","x","y","z","-"].contains(&s)
}

fn match_char(data: &char) -> bool {
    match *data {
        '\x01'...'\x08' |
        '\u{10FFFE}'...'\u{10FFFF}' => true,
        _ => false,
    }
}

fn c_to_string(cs: Vec<(usize,char)>) -> String {
    cs.into_iter().map(|(_, c)| c).collect()
}

pub fn romaji_to_hiragana_safe(romaji: &str) -> String {
    let romaji_chars = UnicodeSegmentation::graphemes(romaji, true).collect::<Vec<&str>>();
    let romaji_len = romaji_chars.len();

    let mut romaji_index = 0;
    let mut window = 1; // 1 to 3
    let mut hiragana = "".to_string();

    // convert 1 to 3 ascii chars to a single hirgana
    // i is starting point, window is length to be looked up
    while romaji_index < romaji_len && romaji_index + window < romaji_len + 1 {
        let romaji_sub_vec = &romaji_chars[romaji_index .. romaji_index + window];
        let romaji_sub_string: String = romaji_sub_vec.join("");
        let romaji_sub: &str = &romaji_sub_string[..];

        let romaji_sub_vec_last = &romaji_chars[romaji_index + window - 1 .. romaji_index + window];
        let romaji_sub_string_last: String = romaji_sub_vec_last.join("");
        let romaji_sub_last: &str = &romaji_sub_string_last[..];

        if !is_romaji(romaji_sub_last) {
            hiragana = format!("{}{}", hiragana, romaji_sub);
            romaji_index += romaji_sub_vec.len();

            window = 1;

        } else {
            // println!("{}", romaji_sub);
            // check for gemminate consonant, if it exists, consume the char
            // add a "っ" and skip the lookup
            if window == 1 && romaji_index + 1 < romaji_len && util::is_consonant(romaji_sub) {
                let romaji_peek_ahead_vec = &romaji_chars[romaji_index + 1 .. romaji_index + 2];
                let romaji_peek_ahead: String = romaji_peek_ahead_vec.join("");

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
                        // keep the unconverted character
                        if romaji_index + window >= romaji_len {
                            hiragana = format!("{}{}", hiragana, romaji_sub);
                        }
                        window += 1;
                    } else {
                        let romaji_sub_vec_first = &romaji_chars[romaji_index .. romaji_index + 1];
                        let romaji_sub_string_first: String = romaji_sub_vec_first.join("");
                        let romaji_sub_first: &str = &romaji_sub_string_first[..];
                        
                        hiragana = format!("{}{}", hiragana, romaji_sub_first);
                        romaji_index += 1;
                        window = 1;
                    }
                },
            }
        }
    }
    
    hiragana.to_string()
}


// this expects everything to be romaji and is destructive
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


mod tests {
    use super::*;
    #[test]
    fn test_romaji_to_hiragana_safe() {
        assert_eq!(romaji_to_hiragana_safe("arigatou"), "ありがとう".to_string());
        assert_eq!(romaji_to_hiragana_safe("ar"), "あr".to_string());
        assert_eq!(romaji_to_hiragana_safe("ari"), "あり".to_string());
        assert_eq!(romaji_to_hiragana_safe("あri"), "あり".to_string());
        assert_eq!(romaji_to_hiragana_safe("hri"), "hり".to_string());
        assert_eq!(romaji_to_hiragana_safe("sme"), "sめ".to_string());
        assert_eq!(romaji_to_hiragana_safe("wあり"), "wあり".to_string());
    }
}
