use std::{
    fmt::{self, Display},
    ops::Range,
};

use crate::ast::{Expr, ExprKind};

#[derive(Debug, Copy, Clone)]
pub struct Span {
    start: usize,
    end: usize,
}

impl Span {
    pub fn new(start: usize, end: usize) -> Self {
        Self { start, end }
    }

    pub fn to(&self, other: Span) -> Span {
        use std::cmp::{max, min};
        Span::new(min(self.start, other.start), max(self.end, other.end))
    }

    pub fn range(&self) -> Range<usize> {
        Range {
            start: self.start,
            end: self.end,
        }
    }
}

impl Display for Span {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let Span { start, end } = self;
        write!(f, "{start}..{end}")
    }
}

pub type PResult<'src, T> = std::result::Result<T, Error>;

#[derive(Debug)]
pub struct Error {
    message: String,
    span: Span,
}

impl Error {
    pub fn new(message: String, span: Span) -> Self {
        Self { message, span }
    }
}

impl std::error::Error for Error {}

impl Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.message.fmt(f)
    }
}

pub struct ErrorPrinter<'a>(pub &'a Error);

impl Display for ErrorPrinter<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let Error { message, span } = self.0;
        write!(f, "{message}, (at {span})")
    }
}

pub struct ExprTreePrinter<'a>(pub &'a Expr);

impl Display for ExprTreePrinter<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        pub struct ExprData<'a>(&'a Expr);

        impl Display for ExprData<'_> {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                match &self.0.kind {
                    ExprKind::Var(i) => write!(f, "VAR ({:?})", i.ident),
                    ExprKind::App(i, _) => write!(f, "APP ({:?})", i.ident),
                }
            }
        }

        fn go(level: usize, expr: &Expr, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(f, "{}", "  ".repeat(level))?;
            writeln!(f, "{} @ {}", ExprData(expr), expr.span)?;
            for children in expr.children() {
                go(level + 1, children, f)?;
            }
            Ok(())
        }

        go(0, self.0, f)
    }
}
