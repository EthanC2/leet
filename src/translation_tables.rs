use phf::phf_map;

/*
    What is Leetspeek?:
*/

/* 
    Level 1: 
    Basic leetspeak replaces some letters with single-digit numbers
*/
pub const LEVEL1: phf::Map<char,&'static str> = phf_map!(
    'a' => "4",
    'b' => "8",
    'e' => "3",
    'g' => "9",
    'i' => "1",
    'l' => "1",
    'o' => "0",
    'r' => "2",
    's' => "5",
    't' => "7",
);

/* 
    Level 2:
    Intermediate leekspeak replaces some letters with single-digit numbers
    and others with multi-character strings that use symbols to represent
    characters. For example, `/ can be used to represent the letter y

    Mapping:
*/

/* 
    Level 3:
*/