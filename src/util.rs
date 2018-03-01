use std::cmp::Ordering::{Equal,Greater,Less};

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

pub fn is_consonant(s: &str) -> bool {
    s.len() == 1 && !["a","e","i","n","o","u","y"].contains(&s)
}
