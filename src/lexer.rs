use std::iter::{self, Peekable};

use crate::{
    ast::{Token, TokenKind},
    util::Span,
};

pub fn lex(src: &str) -> impl Iterator<Item = Token> + '_ {
    let mut chars = src.char_indices().peekable();

    iter::from_fn(move || {
        let (start, curr) = *chars.peek()?;

        let token = |kind, len| Some(Token::new(kind, Span::new(start, start + len)));

        let simple_kind = match curr {
            '(' => TokenKind::LParen,
            ')' => TokenKind::RParen,
            ',' => TokenKind::Comma,
            c if c.is_ascii_digit() => {
                let len = bump_while_next(&mut chars, |c| c.is_ascii_digit());
                return token(TokenKind::Number, len);
            }
            c if c.is_ascii_alphabetic() => {
                let len = bump_while_next(&mut chars, |c| c.is_ascii_alphabetic());
                return token(TokenKind::Ident, len);
            }
            c if c.is_ascii_whitespace() => TokenKind::Whitespace,
            c => {
                chars.next().unwrap();
                return token(TokenKind::ErrorUnexpected(c), c.len_utf8());
            }
        };

        chars.next().unwrap();
        token(simple_kind, 1)
    })
    .filter(|token| token.kind != TokenKind::Whitespace)
    .chain(iter::once(Token::new(
        TokenKind::Eof,
        Span::new(src.len(), src.len()),
    )))
}

fn bump_while_next<I, F>(chars: &mut Peekable<I>, f: F) -> usize
where
    I: Iterator<Item = (usize, char)>,
    F: Fn(char) -> bool,
{
    let mut curr_len = 0;
    while {
        let (_, c) = chars.next().unwrap();
        curr_len += c.len_utf8();
        let next = chars.peek();
        matches!(next, Some((_, c)) if f(*c))
    } {}
    curr_len
}
