use util as u;
use okinawan::util as uu;
// use okinawan::util as uu;
// mod util;

pub fn truncate_chars(s: &str, max_chars: usize) -> &str {
    match s.char_indices().nth(max_chars) {
        None => s,
        Some((idx, _)) => &s[..idx],
    }
}

