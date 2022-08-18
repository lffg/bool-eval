use std::{
    fmt::{self, Display},
    io,
    ops::Range,
};

pub type PResult<'src, T> = std::result::Result<T, Error>;

#[derive(Debug)]
pub struct Error {
    message: String,
    span: Span,
}

impl Error {
    pub fn new(message: impl Into<String>, span: Span) -> Self {
        Self {
            message: message.into(),
            span,
        }
    }

    pub fn report(&self, _text: &str, sink: &mut dyn io::Write) -> io::Result<()> {
        // TODO: Use `_text`.
        let range = self.span.range();
        writeln!(sink, "{} (at {}..{})", self.message, range.start, range.end)
    }
}

impl std::error::Error for Error {}

impl Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.message.fmt(f)
    }
}

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
