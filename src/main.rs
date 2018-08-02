#![feature(test)]

pub mod token;
pub mod tokenref;
pub mod bench;

use token::Token;
use tokenref::TokenRef;

use std::collections::HashSet;
use std::io;

type TokenRefType = TokenRef<'static>;

fn main() {
    let mut largest_muhkus: Vec<(TokenRefType, TokenRefType)> = Vec::new();
    let text = include_str!("../alastalon_salissa.txt");
    let largest_muhku = muhku(text, &mut largest_muhkus);

    println!("Number of word pairs: {}", largest_muhkus.len());
    println!("Muhkeus: {}", largest_muhku);

    for token_pair in largest_muhkus {
        let (foo, bar) = token_pair;
        println!("{:<50} - {:<50} - {:<32}", foo.original_word, bar.original_word, (foo.bitvec | bar.bitvec).count_ones());
    }

}

fn muhku<'a>(text: &'static str, largest_muhkus: &'a mut Vec<(TokenRefType, TokenRefType)>) -> u8 {

    //includes the file as an str constant
    let contents = text;
    let words: Vec<&str> = tokenize(contents);

    let mut largest_single = 0 as u8;

    let tokenset: HashSet<TokenRefType> = words.iter().map(|word| {
        let token = TokenRefType::new(word);
        if token.muhkeus > largest_single {
            largest_single = token.muhkeus;
        }
        token
    }).collect();

    let mut tokens: Vec<TokenRefType> = tokenset.into_iter().collect();
    tokens.sort_by(|a, b| {
        a.muhkeus.cmp(&b.muhkeus)
    });

    let mut largest_muhku = largest_single;
    //let mut largest_muhkus: Vec<(&TokenType, &TokenType)> = Vec::new();

    for (index1, item1) in tokens.iter().enumerate() {
        if item1.muhkeus * 2 < largest_muhku {
            continue
        }
        for index2 in index1..tokens.len() {
            let item2 = &tokens[index2];
            let muhkeusunion = item1.bitvec | item2.bitvec;
            let muhkeus = muhkeusunion.count_ones() as u8;
            if muhkeus > largest_muhku {
                largest_muhkus.clear();
                largest_muhku = muhkeus;
            }
            if muhkeus == largest_muhku {
                largest_muhkus.push((*item1, *item2));
            }
        }

    }

    largest_muhku
}

pub fn muhku_string<'a>(text: &'static str, largest_muhkus: &'a mut Vec<(Token,Token)>) -> u8 {
    type TokenType = Token;
    //includes the file as an str constant
    let contents = text;
    let words: Vec<&str> = tokenize(contents);

    let mut largest_single = 0u8;

    let tokenset: HashSet<TokenType> = words.iter().map(|word| {
        let token = TokenType::new(word);
        if token.muhkeus > largest_single {
            largest_single = token.muhkeus;
        }
        token
    }).collect();

    let mut tokens: Vec<TokenType> = tokenset.into_iter().collect();
    tokens.sort_by(|a, b| {
        a.muhkeus.cmp(&b.muhkeus)
    });


    let mut largest_muhku = largest_single;
    //let mut largest_muhkus: Vec<(&TokenType, &TokenType)> = Vec::new();

    for (index1, item1) in tokens.iter().enumerate() {
        if item1.muhkeus * 2 < largest_muhku {
            continue
        }
        for index2 in index1..tokens.len() {
            let item2 = &tokens[index2];
            let muhkeusunion = item1.bitvec | item2.bitvec;
            let muhkeus = muhkeusunion.count_ones() as u8;
            if muhkeus > largest_muhku {
                largest_muhkus.clear();
                largest_muhku = muhkeus;
            }
            if muhkeus == largest_muhku {
                largest_muhkus.push((item1.clone(), item2.clone()));
            }
        }

    }

    largest_muhku
}


fn tokenize<'a>(text: &'a str) -> Vec<&'a str> {
    text.split(|c: char| !c.is_alphabetic())
    .map(|i| i.trim())
    .filter(|i| !i.is_empty())
    .collect()
}

#[test]
fn test_muhkus() {
    use muhku;
    use muhku_string;

    let mut largest_muhkus: Vec<(TokenRef<'static>, TokenRef<'static>)> = Vec::new();
    let mut largest_muhkus2: Vec<(Token, Token)> = Vec::new();
    {
        let text = include_str!("../alastalon_salissa.txt");
        muhku(text, &mut largest_muhkus);
    }

    {
        type TokenType = Token;

        let text = include_str!("../alastalon_salissa.txt");
        muhku_string(text, &mut largest_muhkus2);
    }
    println!("Ref String");
    for token_pair in largest_muhkus {
        let (foo, bar) = token_pair;
        println!("{:<50} - {:<50} - {:<32}", foo.original_word, bar.original_word, (foo.bitvec | bar.bitvec).count_ones());
    }
    println!("Owned String");
    for token_pair in largest_muhkus2 {
        let (foo, bar) = token_pair;
        println!("{:<50} - {:<50} - {:<32}", foo.original_word, bar.original_word, (foo.bitvec | bar.bitvec).count_ones());
    }

}