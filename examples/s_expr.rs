use notugly::*;

enum SExpr {
    Number(i32),
    Call(String, Vec<SExpr>),
}

impl Format for SExpr {
    fn format(&self) -> Document {
        match self {
            SExpr::Number(n) => text(format!("{n}")),
            SExpr::Call(name, v) => group_with(
                "",
                group(text("(") & text(name) & nest(2, line() & stack(v))) / text(")"),
            ),
        }
    }
}

macro_rules! sexpr {
    ($n:literal) => { SExpr::Number($n) };
    ($name:ident $($args:tt)*) => { SExpr::Call(stringify!($name).into(), vec![$(sexpr!($args)),*])};
    (($name:ident $($args:tt)*)) => { SExpr::Call(stringify!($name).into(), vec![$(sexpr!($args)),*])};
}

fn main() {
    let big_eq = sexpr!(add (mul 2 6) (div (mul 4 (mul 3 2 1)) (add 1 (sub 3 (add 1 1)))));

    println!(
        "{}\n---\n{}\n---\n{}\n---\n{}\n---\n{}",
        big_eq.pretty(80),
        big_eq.pretty(60),
        big_eq.pretty(40),
        nest(20, line() & big_eq.format()).pretty(60),
        big_eq.pretty(20)
    );
}
