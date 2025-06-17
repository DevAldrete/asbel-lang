// This file is a direct adaptation of the `nano_rust.rs` example from the chumsky repository.
// It's intended to provide a stable, working baseline that can be incrementally
// adapted for the ASBEL language.
// Original source: https://github.com/zesterer/chumsky/blob/master/examples/nano_rust.rs

use chumsky::prelude::*;
use std::collections::HashMap;

pub type Span = std::ops::Range<usize>;

#[derive(Clone, Debug, PartialEq)]
pub enum Val {
    Null,
    Bool(bool),
    Num(f64),
    Str(String),
    Func(String),
}

#[derive(Clone, Debug, PartialEq)]
pub enum Expr {
    Error,
    Value(Val),
    List(Vec<Self>),
    Local(String),
    Let(String, Box<Self>, Box<Self>),
    Then(Box<Self>, Box<Self>),
    Binary(Box<Self>, Op, Box<Self>),
    Call(Box<Self>, Vec<Self>),
    If(Box<Self>, Box<Self>, Box<Self>),
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum Token {
    Null,
    Bool(bool),
    Num(String),
    Str(String),
    Op(String),
    Ctrl(char),
    Ident(String),
    Fn,
    Let,
    If,
    Else,
}

impl std::fmt::Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Token::Null => write!(f, "null"),
            Token::Bool(x) => write!(f, "{}", x),
            Token::Num(n) => write!(f, "{}", n),
            Token::Str(s) => write!(f, "{}", s),
            Token::Op(s) => write!(f, "{}", s),
            Token::Ctrl(c) => write!(f, "{}", c),
            Token::Ident(s) => write!(f, "{}", s),
            Token::Fn => write!(f, "fn"),
            Token::Let => write!(f, "let"),
            Token::If => write!(f, "if"),
            Token::Else => write!(f, "else"),
        }
    }
}

pub fn lexer<'a>() -> impl Parser<'a, &'a str, Vec<(Token, Span)>, extra::Err<Simple<'a, char>>> {
    let num = text::int(10)
        .then(just('.').then(text::digits(10)).or_not())
        .to_slice()
        .map(|s: &str| Token::Num(s.to_string()));

    let str_ = just('"')
        .ignore_then(none_of('"').repeated().to_slice())
        .then_ignore(just('"'))
        .map(|s: &str| Token::Str(s.to_string()));

    let op = one_of("+-*/!=".chars())
        .repeated()
        .at_least(1)
        .to_slice()
        .map(|s: &str| Token::Op(s.to_string()));

    let ctrl = one_of("()[],.;".chars()).map(|c| Token::Ctrl(c));

    let ident = text::ident().map(|s: &str| match s {
        "fn" => Token::Fn,
        "let" => Token::Let,
        "if" => Token::If,
        "else" => Token::Else,
        "true" => Token::Bool(true),
        "false" => Token::Bool(false),
        "null" => Token::Null,
        _ => Token::Ident(s.to_string()),
    });

    let token = num
        .or(str_)
        .or(op)
        .or(ctrl)
        .or(ident)
        .recover_with(skip_then_retry_until(
            any().ignored(),
            one_of(" \r\n".chars()).ignored(),
        ));

    let comment = just("//").then(any().and_is(just('\n').not()).repeated());

    token
        .map_with_span(|tok, span| (tok, span))
        .padded_by(comment.repeated())
        .padded()
        .repeated()
        .collect()
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Op {
    Add,
    Sub,
    Mul,
    Div,
    Eq,
    NotEq,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn lex_simple_let_statement() {
        let src = "let a = 5;";
        let tokens = lexer().parse(src).into_output().unwrap();
        let tokens: Vec<_> = tokens.into_iter().map(|(tok, _span)| tok).collect();

        assert_eq!(
            tokens,
            vec![
                Token::Let,
                Token::Ident("a".to_string()),
                Token::Op("=".to_string()),
                Token::Num("5".to_string()),
                Token::Ctrl(';'),
            ]
        );
    }
} 