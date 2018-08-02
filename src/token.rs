use std::hash::{Hash, Hasher};


#[derive(Debug, Clone, Eq)]
pub struct Token{
    pub muhkeus: u8,
    pub bitvec: usize,
    pub original_word: String,
}


impl Token {
    pub fn new(original_word: &str) -> Token {
        /*let lowercasestring: String = original_word.to_string()
            .chars()
            .map(|ch| ch.to_lowercase().next().unwrap())
            .collect();*/
        let bits: usize = get_string_bits(&original_word);
        Token {
            original_word: original_word.to_owned(),
            bitvec: bits,
            muhkeus: bits.count_ones() as u8
        }
    }
}

impl PartialEq for Token {
    fn eq(&self, other: &Self) -> bool {
        self.original_word == other.original_word
    }
}

impl Hash for Token {
    fn hash<H: Hasher>(&self, state: &mut H) {
       self.original_word.hash(state);
   }
}

fn get_string_bits(word: &str) -> usize {
    let mut bits: usize = 0;

    for c in word.chars() {
        let mask: usize =  match c {
            'a'|'A' => 1 << 0,
            'b'|'B' => 1 << 1,
            'c'|'C' => 1 << 2,
            'd'|'D' => 1 << 3,
            'e'|'E' => 1 << 4,
            'f'|'F' => 1 << 5,
            'g'|'G' => 1 << 6,
            'h'|'H' => 1 << 7,
            'i'|'I' => 1 << 8,
            'j'|'J' => 1 << 9,
            'k'|'K' => 1 << 10,
            'l'|'L' => 1 << 11,
            'm'|'M' => 1 << 12,
            'n'|'N' => 1 << 13,
            'o'|'O' => 1 << 14,
            'p'|'P' => 1 << 15,
            'q'|'Q' => 1 << 16,
            'r'|'R' => 1 << 17,
            's'|'S' => 1 << 18,
            't'|'T' => 1 << 19,
            'u'|'U' => 1 << 20,
            'v'|'V' => 1 << 21,
            'w'|'W' => 1 << 22,
            'x'|'X' => 1 << 23,
            'y'|'Y' => 1 << 24,
            'z'|'Z' => 1 << 25,
            'å'|'Å' => 1 << 26,
            'ä'|'Ä' => 1 << 27,
            'ö'|'Ö' => 1 << 28,
            _   => 0
        };
        bits = bits | mask;
    }
    bits
}