extern crate test;

use self::test::Bencher;
use token::Token;
use tokenref::TokenRef;


#[bench]
fn bench_muhku_ref(b: &mut Bencher) {
    use muhku;


    b.iter(|| {
        type TokenType = TokenRef<'static>;
        let mut largest_muhkus: Vec<(TokenType, TokenType)> = Vec::new();
        let text = include_str!("../alastalon_salissa.txt");
        test::black_box(muhku(text, &mut largest_muhkus));
    });
}


#[bench]
fn bench_muhku(b: &mut Bencher) {
    use muhku_string;

    b.iter(|| {
        type TokenType = Token;
        let mut largest_muhkus: Vec<(TokenType, TokenType)> = Vec::new();
        let text = include_str!("../alastalon_salissa.txt");
        test::black_box(muhku_string(text, &mut largest_muhkus));
    });
}

