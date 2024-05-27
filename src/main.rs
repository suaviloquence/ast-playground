use std::time::Instant;

use interface::{BuildAst, Parser, Token};

use crate::interface::Traverse;

mod arena;
mod boxes;
#[cfg(feature = "fuzz")]
mod fuzz;
mod interface;

const TOKENS: &[Token<'static>] = &[
    Token::Str("seq"),
    Token::Str("20"),
    Token::Pipe,
    Token::Str("grep"),
    Token::Str("1"),
    Token::Pipe,
    Token::Str("head"),
    Token::DashDash,
    Token::Str("lines"),
    Token::Equals,
    Token::Str("5"),
];

fn benchmark(tks: impl Iterator<Item = Token<'static>>, builder: impl BuildAst + core::fmt::Debug) {
    println!("Running benchmark with {builder:?}");
    let scanner = tks.peekable();
    let parser = Parser { scanner, builder };

    let start = Instant::now();
    let ast = parser.parse();
    let stop = Instant::now();
    println!("Generated AST in {:?}", stop.duration_since(start));

    let start = Instant::now();
    ast.traverse(|s| eprintln!("found {s}"));
    let stop = Instant::now();
    println!("Traversed AST in {:?}", stop.duration_since(start));
}

fn main() {
    let toks = {
        let start = Instant::now();
        let fuzzer = fuzz::Fuzzer::new(0.9999);
        let tokens = fuzzer.fuzz();
        let stop = Instant::now();
        println!(
            "Generated {} tokens in {:?}",
            tokens.len(),
            stop.duration_since(start)
        );
        tokens
    };

    benchmark(toks.iter().copied(), boxes::BoxBuilder);
    benchmark(toks.iter().copied(), arena::Arena::new());
}
