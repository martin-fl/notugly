use std::collections::VecDeque;

#[derive(Debug, Clone)]
pub(crate) enum FormatDesc {
    Nil,
    Line,
    Text(String),
    Nest(i32, Box<FormatDesc>),
    Cat(Box<FormatDesc>, Box<FormatDesc>),
    Union(Box<FormatDesc>, Box<FormatDesc>),
}

impl FormatDesc {
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

    #[inline(always)]
    pub(crate) fn flatten(&self) -> FormatDesc {
        self.flatten_with(" ")
    }

    #[inline(always)]
    pub(crate) fn best(self, w: i32, k: i32) -> ProcessedFormat {
        be(w, k, VecDeque::from([(0, self)]))
    }

    #[inline(always)]
    pub(crate) fn pretty(self, w: i32) -> ProcessedFormat {
        self.best(w, 0)
    }
}

#[derive(Debug, Clone)]
pub enum ProcessedFormat {
    Nil,
    Text(String, Box<ProcessedFormat>),
    Line(i32, Box<ProcessedFormat>),
}

impl std::fmt::Display for ProcessedFormat {
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

#[inline(always)]
pub(crate) fn better(w: i32, k: i32, x: ProcessedFormat, y: ProcessedFormat) -> ProcessedFormat {
    if x.fits(w - k) {
        x
    } else {
        y
    }
}
