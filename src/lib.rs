mod doc;
use doc::{FormatDesc, ProcessedFormat};

#[derive(Clone, Debug)]
pub struct Formatted(FormatDesc);

impl Formatted {
    pub(crate) fn map(self, f: impl FnOnce(FormatDesc) -> FormatDesc) -> Formatted {
        Formatted(f(self.0))
    }

    pub(crate) fn flatten_with(&self, c: &str) -> Formatted {
        self.clone().map(|x| x.flatten_with(c))
    }

    pub(crate) fn flatten(&self) -> Formatted {
        self.clone().map(|x| x.flatten())
    }
}

pub trait Format {
    fn format(&self) -> Formatted;

    fn pretty(&self, w: i32) -> ProcessedFormat {
        self.format().0.pretty(w)
    }
}

impl Format for Formatted {
    fn format(&self) -> Formatted {
        self.clone()
    }
}

#[inline(always)]
pub fn nil() -> Formatted {
    Formatted(FormatDesc::Nil)
}

#[inline(always)]
pub fn line() -> Formatted {
    Formatted(FormatDesc::Line)
}

#[inline(always)]
pub fn text(s: impl Into<String>) -> Formatted {
    Formatted(FormatDesc::Text(s.into()))
}

#[inline(always)]
pub fn nest(i: i32, x: Formatted) -> Formatted {
    Formatted(FormatDesc::Nest(i, Box::new(x.0)))
}

#[inline(always)]
pub fn cat(x: Formatted, y: Formatted) -> Formatted {
    Formatted(FormatDesc::Cat(Box::new(x.0), Box::new(y.0)))
}

#[inline(always)]
pub(crate) fn union(x: Formatted, y: Formatted) -> Formatted {
    Formatted(FormatDesc::Union(Box::new(x.0), Box::new(y.0)))
}

impl std::ops::BitAnd<Formatted> for Formatted {
    type Output = Formatted;

    #[inline(always)]
    fn bitand(self, rhs: Formatted) -> Self::Output {
        cat(self, rhs)
    }
}

impl std::ops::Add<Formatted> for Formatted {
    type Output = Formatted;

    #[inline(always)]
    fn add(self, rhs: Formatted) -> Self::Output {
        self & text(" ") & rhs
    }
}

impl std::ops::Div<Formatted> for Formatted {
    type Output = Formatted;

    #[inline(always)]
    fn div(self, rhs: Formatted) -> Self::Output {
        self & line() & rhs
    }
}

impl std::ops::Mul<Formatted> for Formatted {
    type Output = Formatted;

    #[inline(always)]
    fn mul(self, rhs: Formatted) -> Self::Output {
        self & union(text(" "), line()) & rhs
    }
}

#[inline(always)]
pub fn group(x: Formatted) -> Formatted {
    union(x.flatten(), x)
}

#[inline(always)]
pub fn group_with(c: &str, x: Formatted) -> Formatted {
    union(x.flatten_with(c), x)
}

#[inline(always)]
pub fn bracket(i: i32, l: impl Into<String>, x: Formatted, r: impl Into<String>) -> Formatted {
    group(text(l) & nest(i, line() & x) / text(r))
}

#[inline(always)]
pub fn fold(xs: &[impl Format], op: impl FnMut(Formatted, Formatted) -> Formatted) -> Formatted {
    xs.iter().map(Format::format).reduce(op).unwrap_or(nil())
}

#[inline(always)]
pub fn spread(xs: &[impl Format]) -> Formatted {
    fold(xs, |lhs, rhs| lhs + rhs)
}

#[inline(always)]
pub fn stack(xs: &[impl Format]) -> Formatted {
    fold(xs, |lhs, rhs| lhs / rhs)
}

pub fn fill(xs: &[Formatted]) -> Formatted {
    match &xs[..] {
        [] => nil(),
        [x] => x.clone(),
        [x, y, z @ ..] => {
            let (x, y) = (x.clone(), y.clone());
            let z1 = [y.flatten()]
                .iter()
                .chain(z.iter())
                .cloned()
                .collect::<Vec<_>>();
            let z2 = [y].iter().chain(z.iter()).cloned().collect::<Vec<_>>();
            union(x.flatten() + fill(&z1), x / fill(&z2))
        }
    }
}
