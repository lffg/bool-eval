use std::{
    cmp,
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

pub type PResult<T> = std::result::Result<T, Error>;

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

pub struct ErrorPrinter<'a, 'src>(pub &'a Error, pub &'src str);

impl Display for ErrorPrinter<'_, '_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        use ansi_term::Color::{Blue, Red};
        use unicode_width::UnicodeWidthStr;

        fn line_bounds(src: &str, Span { start, end }: Span) -> (usize, usize) {
            let left = src[..start].rfind('\n').map(|p| p + 1).unwrap_or(0);
            let right = src[end..]
                .find('\n')
                .map(|p| start + p + 1)
                .unwrap_or(src.len());
            (left, right)
        }

        fn src_sections(src: &str, span: Span) -> (&str, &str, &str) {
            let (lo, hi) = line_bounds(src, span);
            let left = &src[lo..span.start];
            let offending = &src[span.range()];
            let right = &src[span.end..hi];
            (left, offending, right)
        }

        let Self(Error { message, span }, src) = self;
        let (before, offending, after) = src_sections(src, *span);

        let spaces = " ".repeat(UnicodeWidthStr::width(before));
        let arrows = "^".repeat(cmp::max(1, UnicodeWidthStr::width(offending)));

        let a = Blue.paint("-->");
        let p = Blue.paint("|");

        writeln!(f, "{a} Error: {message} (at {span})")?;
        writeln!(f, " {p}")?;
        writeln!(f, " {p} {}{}{}", before, Red.paint(offending), after)?;
        writeln!(f, " {p} {}{}", spaces, Blue.paint(arrows))?;
        writeln!(f, " {p}")?;

        Ok(())
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
