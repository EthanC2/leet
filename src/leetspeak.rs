use crate::translation_tables::{LEVEL1, LEVEL2, LEVEL3};

pub fn translate(text: &str, level: &u8) -> String {
    let translation_table;
    match level {
        1 => translation_table = LEVEL1,
        2 => translation_table = LEVEL2,
        3 => translation_table = LEVEL3,
        _ => unreachable!("Range of `level` is bound from 1..=3 by CLAP"),
    }

    text.chars()
        .into_iter()
        .fold(String::from(""), |accum, c| 
        match translation_table.get(&c) {
            Some(s) => accum + *s,
            None => accum + c.to_string().as_str()
            }
        )
}