use crate::translation_tables;

pub fn translate(text: &str, level: &u8) -> String {
    let translation_table = translation_tables::LEVEL1;
    let chars = text.chars();

    chars.into_iter()
         .fold(String::from(""),
          |accum, c| 
          match translation_table.get(&c) {
            Some(s) => accum + *s,
            None => accum + c.to_string().as_str()
           }
         )
}