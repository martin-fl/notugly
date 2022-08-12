//! notugly
//! =============
//!
//! Simple and generic pretty-printing library, heavily based on
//! [A prettier printer](https://homepages.inf.ed.ac.uk/wadler/papers/prettier/prettier.pdf).
//!
//! ## How to use
//!
//! To define pretty-printing for a type, implement the [Format] trait using the various utility functions:
//! - [nil] for a null element
//! - [text] to create a string into a document
//! - [nest] to indent a document
//! - [cat] to concatenate two documents
//! - [group] and [group_with] to add the flattened layout as an alternative.
//! - [fold], [spread], [stack] and [fill] to collapse a list of documents in various ways
//!
//!
//! To make it easier to define a structure, some operators are defined:
//! - `x & y == cat(x,y)`
//! - `x + y == x & text(" ") & y`
//! - `x / y == x & line() & y`
//! - `x * y == x & group(line) & y`
//!
//! See the `examples/` directory for fully-featured examples.
//!
//! ## Example : S-Expressions
//!
//! ```rust
//! use notugly::*;
//!
//! enum SExpr {
//!     Number(i32),
//!     Call(String, Vec<SExpr>),
//! }
//!
//! impl Format for SExpr {
//!     fn format(&self) -> Document {
//!         match self {
//!             SExpr::Number(n) => text(format!("{n}")),
//!             SExpr::Call(name, v) => group_with(
//!                 "",
//!                 group(text("(") & text(name) & nest(2, line() & stack(v))) / text(")"),
//!             ),
//!         }
//!     }
//! }
//! fn main() {
//!     let big_eq = sexpr!(add (mul 2 6) (div (mul 4 (mul 3 2 1)) (add 1 (sub 3 (add 1 1)))));
//!
//!     println!("{}", big_eq.pretty(40));
//! }
//! ```
mod format;
use format::{FormatDesc, ProcessedFormat};

/// Opaque type representating a set of possible layouts for a document.
#[derive(Clone, Debug)]
pub struct Document(FormatDesc);

impl Document {
    pub(crate) fn map(self, f: impl FnOnce(FormatDesc) -> FormatDesc) -> Document {
        Document(f(self.0))
    }

    pub(crate) fn flatten_with(&self, c: &str) -> Document {
        self.clone().map(|x| x.flatten_with(c))
    }

    pub(crate) fn flatten(&self) -> Document {
        self.clone().map(|x| x.flatten())
    }
}

/// Given the `format` method, telling how to turn a `Self` into a [Document],
/// provides the `pretty` method to choose the best layout.
pub trait Format {
    fn format(&self) -> Document;

    fn pretty(&self, w: i32) -> ProcessedFormat {
        self.format().0.pretty(w)
    }
}

impl Format for Document {
    fn format(&self) -> Document {
        self.clone()
    }
}

/// Produces a null document.
#[inline(always)]
pub fn nil() -> Document {
    Document(FormatDesc::Nil)
}

/// Produces a newline marker.
#[inline(always)]
pub fn line() -> Document {
    Document(FormatDesc::Line)
}

/// Transforms text into a document.
#[inline(always)]
pub fn text(s: impl Into<String>) -> Document {
    Document(FormatDesc::Text(s.into()))
}

/// Indent the given document with `i` spaces.
#[inline(always)]
pub fn nest(i: i32, x: Document) -> Document {
    Document(FormatDesc::Nest(i, Box::new(x.0)))
}

/// Concatenates two documents.
#[inline(always)]
pub fn cat(x: Document, y: Document) -> Document {
    Document(FormatDesc::Cat(Box::new(x.0), Box::new(y.0)))
}

/// Marks `x` and `y` as alternative layouts, `x` taking precedence over `y`.
/// `x` and `y` must flatten to the same layout, i.e.
/// `x.flatten().to_string() == y.flatten().to_string()`
#[inline(always)]
pub(crate) fn union(x: Document, y: Document) -> Document {
    Document(FormatDesc::Union(Box::new(x.0), Box::new(y.0)))
}

impl std::ops::BitAnd<Document> for Document {
    type Output = Document;

    #[inline(always)]
    fn bitand(self, rhs: Document) -> Self::Output {
        cat(self, rhs)
    }
}

impl std::ops::Add<Document> for Document {
    type Output = Document;

    #[inline(always)]
    fn add(self, rhs: Document) -> Self::Output {
        self & text(" ") & rhs
    }
}

impl std::ops::Div<Document> for Document {
    type Output = Document;

    #[inline(always)]
    fn div(self, rhs: Document) -> Self::Output {
        self & line() & rhs
    }
}

impl std::ops::Mul<Document> for Document {
    type Output = Document;

    #[inline(always)]
    fn mul(self, rhs: Document) -> Self::Output {
        self & union(text(" "), line()) & rhs
    }
}

/// Adds the flattened layout (everything compressed on one line) as
/// an alternative layout to a document.
#[inline(always)]
pub fn group(x: Document) -> Document {
    union(x.flatten(), x)
}

/// Adds the flattened layout (everything compressed on one line, newlines being replace by the `c` string)
/// as an alternative layout to a document.
#[inline(always)]
pub fn group_with(c: &str, x: Document) -> Document {
    union(x.flatten_with(c), x)
}

/// Convenience function for the common operation of delimiting a document.
///
/// The `x` document will be indented with `i` spaces, and enclosed by the `l` and `r` elements.
#[inline(always)]
pub fn bracket(i: i32, l: impl Into<String>, x: Document, r: impl Into<String>) -> Document {
    group(text(l) & nest(i, line() & x) / text(r))
}

/// Collapses a list of documents according to `op`. If the slice is empty,
/// returns [nil].
#[inline(always)]
pub fn fold(xs: &[impl Format], op: impl FnMut(Document, Document) -> Document) -> Document {
    xs.iter().map(Format::format).reduce(op).unwrap_or(nil())
}

/// Collapses a list of documents and inserts a space between every element of the slice.
#[inline(always)]
pub fn spread(xs: &[impl Format]) -> Document {
    fold(xs, |lhs, rhs| lhs + rhs)
}

/// Collapses a list of documents and inserts a newline between every element of the slice.
#[inline(always)]
pub fn stack(xs: &[impl Format]) -> Document {
    fold(xs, |lhs, rhs| lhs / rhs)
}

/// Collapses a list of document and recursively adds the alternative layouts of
/// using a space or a newline between each document
///
/// See page 14 of [A prettier printer](https://homepages.inf.ed.ac.uk/wadler/papers/prettier/prettier.pdf).
pub fn fill(xs: &[impl Format]) -> Document {
    match &xs[..] {
        [] => nil(),
        [x] => x.format(),
        [x, y, z @ ..] => {
            let x = x.format();
            let z1 = [y]
                .iter()
                .map(|y| y.format().flatten())
                .chain(z.iter().map(Format::format))
                .collect::<Vec<_>>();
            let z2 = [y]
                .iter()
                .map(|y| y.format())
                .chain(z.iter().map(Format::format))
                .collect::<Vec<_>>();
            union(x.flatten() + fill(&z1), x / fill(&z2))
        }
    }
}
