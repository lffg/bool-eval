use std::iter::Peekable;

use crate::{
    ast::{Expr, ExprKind, Ident, Program, Token, TokenKind},
    evaluator::MAX_ARG_COUNT,
    util::{Error, PResult, Span},
};

pub fn parse(src: &str, tokens: impl Iterator<Item = Token>) -> PResult<Program> {
    let mut p = Parser {
        tokens: tokens.peekable(),
        src,
    };

    p.parse_program()
}

//
// Main parser implementation.
//

impl<'src, T> Parser<'src, T>
where
    T: Iterator<Item = Token>,
{
    fn parse_program(&mut self) -> PResult<Program> {
        let args = self.parse_control()?;
        let expr = self.parse_expr()?;
        self.consume(TokenKind::Eof)?;
        Ok(Program { expr, args })
    }

    fn parse_control(&mut self) -> PResult<Vec<bool>> {
        let (arg_count, arg_count_span) = self.parse_number()?;
        if arg_count > MAX_ARG_COUNT {
            return Err(self.error_at(
                format!("can't declare more than {MAX_ARG_COUNT} params"),
                arg_count_span,
            ));
        }
        let mut args = Vec::new();
        for _ in 1..=arg_count {
            if !self.is(TokenKind::Number) {
                let kind = self.peek().kind;
                return Err(self.error_at_next(format!(
                    "expected next bit, instead got token of kind `{kind:?}`"
                )));
            }
            match self.parse_number()? {
                (bit @ (0 | 1), _) => args.push(bit == 1),
                (_, span) => return Err(self.error_at("invalid bit, must be `0` or `1`", span)),
            }
        }
        Ok(args)
    }

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

    fn parse_number(&mut self) -> PResult<(usize, Span)> {
        let number_t = self.consume(TokenKind::Number)?;
        number_t
            .lexme(self.src)
            .parse()
            .map(|n| (n, number_t.span))
            .map_err(|_| self.error_at("unparsable number", number_t.span))
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
        self.tokens.next().expect("bug: parser advanced past eof")
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
            let actual = self.peek().kind;
            Err(self.error_at_next(format!(
                "expected token of kind `{kind:?}`, instead got `{actual:?}`"
            )))
        }
    }

    /// Checks if the next token matches the given [`TokenKind`] list. If not,
    /// it returns an error.
    fn expect_of(&mut self, list: &[TokenKind]) -> PResult<TokenKind> {
        self.is_of(list).ok_or_else(|| {
            let actual = self.peek().kind;
            self.error_at_next(format!(
                "expected token of kind in `{list:?}`, instead got `{actual:?}`"
            ))
        })
    }

    /// Checks if the next token matches the given [`TokenKind`]. If so, it
    /// advances and returns the previous token. Otherwise, returns an error.
    fn consume(&mut self, kind: TokenKind) -> PResult<Token> {
        self.expect(kind)?;
        Ok(self.next())
    }

    /// Constructs a new error.
    #[must_use]
    fn error_at(&mut self, message: impl Into<String>, span: Span) -> Error {
        Error::new(message.into(), span)
    }

    /// Constructs a new error pointing to the next span.
    #[must_use]
    fn error_at_next(&mut self, message: impl Into<String>) -> Error {
        Error::new(message.into(), self.peek().span)
    }
}
