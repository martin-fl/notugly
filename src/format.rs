use std::collections::VecDeque;

/// Describes the different ways to assemble a document.
///
/// See pages 2 & 6 of [A prettier printer](https://homepages.inf.ed.ac.uk/wadler/papers/prettier/prettier.pdf),
#[derive(Debug, Clone)]
pub(crate) enum FormatDesc {
    /// Does nothing, the empty word
    Nil,
    /// Inserts a newline
    Line,
    /// Inserts the given string
    Text(String),
    /// Indent the given document by the given number of spaces
    Nest(i32, Box<FormatDesc>),
    /// Concatenates the given documents
    Cat(Box<FormatDesc>, Box<FormatDesc>),
    /// Represents a set of possible layouts. The two documents are required
    /// to flatten to the same layout as an invariant.
    Union(Box<FormatDesc>, Box<FormatDesc>),
}

impl FormatDesc {
    /// Removes indentation and replaces newlines with `c`.
    #[inline(always)]
    pub(crate) fn flatten_with(&self, c: &str) -> FormatDesc {
        match self {
            FormatDesc::Nil => FormatDesc::Nil,
            FormatDesc::Line => FormatDesc::Text(c.into()),
            FormatDesc::Text(s) => FormatDesc::Text(s.clone()),
            FormatDesc::Nest(_, x) => x.flatten_with(c),
            FormatDesc::Cat(x, y) => {
                FormatDesc::Cat(Box::new(x.flatten_with(c)), Box::new(y.flatten_with(c)))
            }
            FormatDesc::Union(x, _) => x.flatten_with(c),
        }
    }

    /// Removes indentation and replaces newlines with a single space
    #[inline(always)]
    pub(crate) fn flatten(&self) -> FormatDesc {
        self.flatten_with(" ")
    }

    /// Determines the best layout that fits within `w` columns, `k` of which being already used,
    /// and transforms it into a [ProcessedFormat] for easier printing.
    #[inline(always)]
    pub(crate) fn best(self, w: i32, k: i32) -> ProcessedFormat {
        be(w, k, VecDeque::from([(0, self)]))
    }

    /// Determines the best layout that fits within `w` columns,
    /// and transforms it into a [ProcessedFormat] for easier printing.
    #[inline(always)]
    pub(crate) fn pretty(self, w: i32) -> ProcessedFormat {
        self.best(w, 0)
    }
}

/// Simplified representation of a formatted document
#[derive(Debug, Clone)]
pub enum ProcessedFormat {
    Nil,
    Text(String, Box<ProcessedFormat>),
    Line(i32, Box<ProcessedFormat>),
}

impl std::fmt::Display for ProcessedFormat {
    /// Corresponds to the `layout` function of
    /// [A prettier printer](https://homepages.inf.ed.ac.uk/wadler/papers/prettier/prettier.pdf)
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ProcessedFormat::Nil => Ok(()),
            ProcessedFormat::Text(s, x) => write!(f, "{s}{x}"),
            ProcessedFormat::Line(i, x) => {
                write!(f, "\n{}{}", " ".repeat((*i).try_into().unwrap_or(0)), x)
            }
        }
    }
}

impl ProcessedFormat {
    /// Simply says if the documents fits in the remaining space or not.
    pub(crate) fn fits(&self, w: i32) -> bool {
        if w < 0 {
            return false;
        }

        match self {
            ProcessedFormat::Nil | ProcessedFormat::Line(_, _) => true,
            ProcessedFormat::Text(s, x) => x.fits(w - s.len() as i32),
        }
    }
}

/// Given a list of (indentation, document) pairs, chooses the best layout possible for the given width `w`
/// and remaining space `k`.
pub(crate) fn be(w: i32, k: i32, mut z: VecDeque<(i32, FormatDesc)>) -> ProcessedFormat {
    match z.pop_front() {
        None => ProcessedFormat::Nil,
        Some((_, FormatDesc::Nil)) => be(w, k, z),
        Some((i, FormatDesc::Cat(x, y))) => {
            z.push_front((i, *y));
            z.push_front((i, *x));
            be(w, k, z)
        }
        Some((i, FormatDesc::Nest(j, x))) => {
            z.push_front((i + j, *x));
            be(w, k, z)
        }
        Some((_, FormatDesc::Text(s))) => {
            let slen = s.len();
            ProcessedFormat::Text(s, Box::new(be(w, slen as i32 + k, z)))
        }
        Some((i, FormatDesc::Line)) => ProcessedFormat::Line(i, Box::new(be(w, i, z))),
        Some((i, FormatDesc::Union(x, y))) => {
            let mut z1 = z;
            let mut z2 = z1.clone();
            z1.push_front((i, *x));
            z2.push_front((i, *y));
            better(w, k, be(w, k, z1), be(w, k, z2))
        }
    }
}

/// Returns `x` if it fits, othewise return `y`. Both documents are supposed to flatten to the same layout.
#[inline(always)]
pub(crate) fn better(w: i32, k: i32, x: ProcessedFormat, y: ProcessedFormat) -> ProcessedFormat {
    if x.fits(w - k) {
        x
    } else {
        y
    }
}
