use standard::tables::*;
use util;

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
