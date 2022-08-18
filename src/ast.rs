use crate::util::Span;

#[derive(Debug, Clone)]
pub struct Expr {
    pub kind: ExprKind,
    pub span: Span,
}

#[derive(Debug, Clone)]
pub enum ExprKind {
    Var(Ident),
    App(Ident, Vec<Expr>),
}

#[derive(Debug, Clone)]
pub struct Ident {
    pub ident: String,
    pub span: Span,
}

#[derive(Debug, Copy, Clone)]
pub struct Token {
    pub kind: TokenKind,
    pub span: Span,
    fabricated: bool,
}

impl Token {
    pub fn new(kind: TokenKind, span: Span) -> Self {
        Self {
            kind,
            span,
            fabricated: false,
        }
    }

    pub fn lexme<'src>(&self, src: &'src str) -> &'src str {
        if self.fabricated {
            self.kind.fabricated_lexme()
        } else {
            &src[self.span.range()]
        }
    }

    pub fn fabricate(kind: TokenKind, span: Span) -> Self {
        Self {
            kind,
            span,
            fabricated: true,
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum TokenKind {
    Ident,
    LParen,
    RParen,
    Comma,
    Whitespace,
    Eof,
    ErrorUnexpected(char),
}

impl TokenKind {
    pub fn fabricated_lexme(&self) -> &'static str {
        match self {
            TokenKind::Ident => "fabricated",
            TokenKind::LParen => "(",
            TokenKind::RParen => ")",
            TokenKind::Comma => ",",
            TokenKind::Whitespace => "",
            TokenKind::Eof => "",
            TokenKind::ErrorUnexpected(_) => "",
        }
    }
}
