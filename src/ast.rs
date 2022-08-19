use crate::util::Span;

#[derive(Debug)]
pub struct Program {
    pub expr: Expr,
    pub args: Vec<bool>,
}

#[derive(Debug)]
pub struct Expr {
    pub kind: ExprKind,
    pub span: Span,
}

impl Expr {
    pub fn children(&self) -> Box<dyn Iterator<Item = &Expr> + '_> {
        match &self.kind {
            ExprKind::App(_, args) => Box::new(args.iter()),
            _ => Box::new(std::iter::empty()),
        }
    }
}

#[derive(Debug)]
pub enum ExprKind {
    Var(Ident),
    App(Ident, Vec<Expr>),
}

#[derive(Debug)]
pub struct Ident {
    pub ident: String,
    pub span: Span,
}

#[derive(Debug, Copy, Clone)]
pub struct Token {
    pub kind: TokenKind,
    pub span: Span,
}

impl Token {
    pub fn new(kind: TokenKind, span: Span) -> Self {
        Self { kind, span }
    }

    pub fn lexme<'src>(&self, src: &'src str) -> &'src str {
        &src[self.span.range()]
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum TokenKind {
    Ident,
    Number,
    LParen,
    RParen,
    Comma,
    Whitespace,
    Eof,
    ErrorUnexpected(char),
}
