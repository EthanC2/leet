use phf::phf_map;

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
    Intermediate leekspeak replaces most letters; the replaced letters
    are substituted for either single-digit numbers multi-character strings that 
    use symbols to represent characters. For example, `/ can be used to represent 
    the letter y
*/
pub const LEVEL2: phf::Map<char,&'static str> = phf_map!(
    'a' => "@",
    'b' => "8",
    'c' => "<",
    'd' => "|)",
    'e' => "3",
    'g' => "9",
    'h' => "#",
    'i' => "1",
    'j' => "_|",
    'k' => "k",
    'l' => "1",
    'o' => "0",
    'q' => "0_",
    'r' => "2",
    's' => "5",
    't' => "7",
    'u' => "v",
    'v' => "\\/",
    'w' => "vv",
    'y' => "`/",
    'z' => "7_",
);

/* 
    Level 3:
    Full leekspeak replaces all letters; the replaced letters
    are substituted for either single-digit numbers multi-character strings that 
    use symbols to represent characters. For example, `/ can be used to represent 
    the letter y
*/
pub const LEVEL3: phf::Map<char,&'static str> = phf_map!(
    'a' => "@",
    'b' => "/3",
    'c' => "(",
    'd' => "cl",
    'e' => "&",
    'f' => "/=",
    'g' => "(_+",
    'h' => "/-/",
    'i' => "!",
    'j' => "_]",
    'k' => "|<",
    'l' => "|_",
    'm' => "/V\\",
    'n' => "|\\|",
    'o' => "()",
    'p' => "|>",
    'q' => "0_",
    'r' => "I2",
    's' => "$",
    't' => "+",
    'u' => "v",
    'v' => "\\|",
    'w' => "vv",
    'x' => "}{",
    'y' => "`/",
    'z' => "7_",
);