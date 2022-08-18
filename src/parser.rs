use std::iter::Peekable;

use crate::{
    ast::{Expr, ExprKind, Ident, Token, TokenKind},
    util::{Error, PResult, Span},
};

pub fn parse(src: &str, tokens: impl Iterator<Item = Token>) -> PResult<Expr> {
    let mut p = Parser {
        tokens: tokens.peekable(),
        src,
    };

    let expr = p.parse_expr()?;
    p.expect(TokenKind::Eof)?;
    Ok(expr)
}

//
// Main parser implementation.
//

impl<'src, T> Parser<'src, T>
where
    T: Iterator<Item = Token>,
{
    fn parse_expr(&mut self) -> PResult<Expr> {
        let ident = self.parse_ident()?;
        if self.is(TokenKind::LParen) {
            self.parse_app_expr(ident)
        } else {
            Ok(Expr {
                span: ident.span,
                kind: ExprKind::Var(ident),
            })
        }
    }

    fn parse_app_expr(&mut self, ident: Ident) -> PResult<Expr> {
        self.consume(TokenKind::LParen)?;
        let args = self.parse_app_args()?;
        let r_paren = self.consume(TokenKind::RParen)?;

        Ok(Expr {
            span: ident.span.to(r_paren.span),
            kind: ExprKind::App(ident, args),
        })
    }

    fn parse_app_args(&mut self) -> PResult<Vec<Expr>> {
        let mut args = Vec::new();
        loop {
            if self.is(TokenKind::RParen) {
                break;
            }
            args.push(self.parse_expr()?);
            match self.expect_of(&[TokenKind::Comma, TokenKind::RParen])? {
                TokenKind::Comma => {
                    self.next();
                    continue;
                }
                TokenKind::RParen => break,
                _ => unreachable!(),
            }
        }
        Ok(args)
    }

    fn parse_ident(&mut self) -> PResult<Ident> {
        let ident_t = self.consume(TokenKind::Ident)?;
        let ident = ident_t.lexme(self.src).into();
        Ok(Ident {
            span: ident_t.span,
            ident,
        })
    }
}

//
// Parser utilities.
//

struct Parser<'src, T>
where
    T: Iterator<Item = Token>,
{
    tokens: Peekable<T>,
    src: &'src str,
}

impl<'src, T> Parser<'src, T>
where
    T: Iterator<Item = Token>,
{
    /// Advances to the next token, returning the advanced token.
    fn next(&mut self) -> Token {
        self.tokens.next().expect("parser advanced past eof")
    }

    /// Peeks into the next token.
    fn peek(&mut self) -> Token {
        self.tokens.peek().copied().unwrap_or_else(|| Token {
            kind: TokenKind::Eof,
            span: Span::new(self.src.len(), self.src.len()),
        })
    }

    /// Checks if the next token matches the given [`TokenKind`].
    fn is(&mut self, kind: TokenKind) -> bool {
        self.peek().kind == kind
    }

    /// Checks if the next token matches the given [`TokenKind`] list.
    fn is_of(&mut self, list: &[TokenKind]) -> Option<TokenKind> {
        for &kind in list {
            if self.is(kind) {
                return Some(kind);
            }
        }
        None
    }

    /// Checks if the next token matches the given [`TokenKind`]. If not, it
    /// returns an error.
    fn expect(&mut self, kind: TokenKind) -> PResult<()> {
        if self.is(kind) {
            Ok(())
        } else {
            Err(Error::new(
                format!(
                    "expected token of kind `{kind:?}`, instead got `{:?}`",
                    self.peek().kind
                ),
                self.peek().span,
            ))
        }
    }

    /// Checks if the next token matches the given [`TokenKind`] list. If not,
    /// it returns an error.
    fn expect_of(&mut self, list: &[TokenKind]) -> PResult<TokenKind> {
        self.is_of(list).ok_or_else(|| {
            Error::new(
                format!(
                    "expected token of kind in `{list:?}`, instead got `{:?}`",
                    self.peek().kind
                ),
                self.peek().span,
            )
        })
    }

    /// Checks if the next token matches the given [`TokenKind`]. If so, it
    /// advances and returns the previous token. Otherwise, returns an error.
    fn consume(&mut self, kind: TokenKind) -> PResult<Token> {
        self.expect(kind)?;
        Ok(self.next())
    }
}
