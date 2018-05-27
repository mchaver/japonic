use std::cmp::Ordering::{Equal,Greater,Less};

pub fn truncate_chars(s: &str, max_chars: usize) -> &str {
    match s.char_indices().nth(max_chars) {
        None => s,
        Some((idx, _)) => &s[..idx],
    }
}

pub fn split_chars_at(s: &str, split: usize) -> (String,String) {
    let length = s.chars().count();

    let l_sub_vec = &s.chars().collect::<Vec<_>>()[0 .. split];
    let l: String = l_sub_vec.into_iter().collect();

    let r_sub_vec = &s.chars().collect::<Vec<_>>()[split .. s.chars().count()];
    let r: String = r_sub_vec.into_iter().collect();
    (l,r)
}


// the table provided must be sorted or it may not return the correct value
pub fn lookup(s: &str, table: &'static [(&str,&str)]) -> Option<usize> {
    match table.binary_search_by(|&(key, _)| {
        if s == key { Equal }
        else if key < s { Less }
        else { Greater }
    }) {
        Ok(i) => Some(i),
        Err(_) => None,
    }
}

pub fn lookup_string(s: &str, table: &'static [(&str,&str)]) -> Option<String> {
    match table.binary_search_by(|&(key, _)| {
        if s == key { Equal }
        else if key < s { Less }
        else { Greater }
    }) {
        Ok(i) => Some(table[i].1.to_string()),
        Err(_) => None,
    }
}

pub fn lookups(s: &str,x: &str, table: &'static [(&str, &'static [(&str,&str)])]) -> Option<usize> {
    match table.binary_search_by(|&(key, _)| {
        if s == key { Equal }
        else if key < s { Less }
        else { Greater }
    }) {
        Ok(i) => lookup(x, table[i].1),
        Err(_) => None,
    }
}

pub fn lookups_string(s: &str,x: &str, table: &'static [(&str, &'static [(&str,&str)])]) -> Option<String> {
    match table.binary_search_by(|&(key, _)| {
        if s == key { Equal }
        else if key < s { Less }
        else { Greater }
    }) {
        Ok(i) => lookup_string(x, table[i].1),
        Err(_) => None,
    }
}

pub fn is_consonant(s: &str) -> bool {
    s.len() == 1 && !["-","a","e","i","n","o","u","y"].contains(&s)
}

pub fn hiragana_has_consonant(s: &str) -> bool {
    s.len() == 1 && !["a","e","i","n","o","u","y"].contains(&s)
}

pub fn hiragana_is_bilabial(s: &str) -> bool {
    s.chars().count() == 1 && ["ば","ぱ","び","ぴ","ぶ","ぷ","べ","ぺ","ぼ","ぽ","ま","み","む","め","も"].contains(&s)
}

pub fn hiragana_merges_n(s: &str) -> bool {
    s.chars().count() == 1 && ["ぁ","あ","ぃ","い","ぅ","う","ぇ","え","ぉ","お","ゃ","や","ゅ","ゆ","ょ","よ","ん"].contains(&s)
}

pub fn is_hiragana(c: char) -> bool {
    c >= '\u{3040}' && c <= '\u{309F}'
}

pub fn is_katakana(c: char) -> bool {
    c >= '\u{30A0}' && c <= '\u{30FF}'
}

pub const HIRAGANA_TO_VOWEL_TABLE: &'static [(&str, &str)] = &[
    ("ぁ","あ"),("あ","あ"),("ぃ","い"),("い","い"),("ぅ","う"),("う","う"),("ぇ","え"),("え","え"),("ぉ","お"),("お","お"),("か","あ"),("が","あ"),("き","い"),("ぎ","い"),("く","う"),("ぐ","う"),("け","え"),("げ","え"),("こ","お"),("ご","お"),("さ","あ"),("ざ","あ"),("し","い"),("じ","い"),("す","う"),("ず","う"),("せ","え"),("ぜ","え"),("そ","お"),("ぞ","お"),("た","あ"),("だ","あ"),("ち","い"),("ぢ","い"),("つ","う"),("づ","う"),("て","え"),("で","え"),("と","お"),("ど","お"),("な","あ"),("に","い"),("ぬ","う"),("ね","え"),("の","お"),("は","あ"),("ば","あ"),("ぱ","あ"),("ひ","い"),("び","い"),("ぴ","い"),("ふ","う"),("ぶ","う"),("ぷ","う"),("へ","え"),("べ","え"),("ぺ","え"),("ほ","お"),("ぼ","お"),("ぽ","お"),("ま","あ"),("み","い"),("む","う"),("め","え"),("も","お"),("ゃ","あ"),("や","あ"),("ゅ","う"),("ゆ","う"),("ょ","お"),("よ","お"),("ら","あ"),("り","い"),("る","う"),("れ","え"),("ろ","お"),("わ","あ"),("ゐ","い"),("ゑ","え"),("を","お"),("ゔ","う"),("ゕ","あ"),("ゖ","え")
];

pub const HIRAGANA_TO_GEMINATE_TABLE: &'static [(&str, &str)] = &[
    ("ぁ",""),("あ",""),("ぃ",""),("い",""),("ぅ",""),("う",""),("ぇ",""),("え",""),("ぉ",""),("お",""),("か","k"),("が","g"),("き","k"),("ぎ","g"),("く","k"),("ぐ","g"),("け","k"),("げ","g"),("こ","k"),("ご","g"),("さ","s"),("ざ","z"),("し","s"),("じ","j"),("す","s"),("ず","z"),("せ","s"),("ぜ","z"),("そ","s"),("ぞ","z"),("た","t"),("だ","d"),("ち","t"),("ぢ","d"),("つ","t"),("づ","d"),("て","t"),("で","d"),("と","t"),("ど","d"),("な","n"),("に","n"),("ぬ","n"),("ね","n"),("の","n"),("は","h"),("ば","b"),("ぱ","p"),("ひ","h"),("び","b"),("ぴ","p"),("ふ","f"),("ぶ","b"),("ぷ","p"),("へ","h"),("べ","b"),("ぺ","b"),("ほ","h"),("ぼ","b"),("ぽ","p"),("ま","m"),("み","m"),("む","m"),("め","m"),("も","m"),("ゃ","y"),("や","y"),("ゅ","y"),("ゆ","y"),("ょ","y"),("よ","y"),("ら","r"),("り","r"),("る","r"),("れ","r"),("ろ","r"),("わ","w"),("ゐ","w"),("ゑ","w"),("を","w"),("ゔ","v"),("ゕ","k"),("ゖ","k")
];

pub fn chars_split(s: &str, split: usize) -> (String,String) {
    let l_sub_vec = &s.chars().collect::<Vec<_>>()[0 .. split];
    let l: String = l_sub_vec.into_iter().collect();

    let r_sub_vec = &s.chars().collect::<Vec<_>>()[split .. s.chars().count()];
    let r: String = r_sub_vec.into_iter().collect();

    (l,r)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_chars_split() {
        assert_eq!(chars_split("かた",1), ("か".to_string(), "た".to_string()));
        assert_eq!(chars_split("あい",1), ("あ".to_string(), "い".to_string()));
        assert_eq!(chars_split("あ",1), ("あ".to_string(), "".to_string()));
    }
}
