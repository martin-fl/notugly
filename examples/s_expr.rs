use notugly::*;

enum SExpr {
    Number(i32),
    Call(String, Vec<SExpr>),
}

impl Format for SExpr {
    fn format(&self) -> Formatted {
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
    let add_2_3 = sexpr!(add (mul 2 6) (div (mul 4 (mul 3 2 1)) (add 1 (sub 3 (add 1 1)))));

    println!(
        "{}\n---\n{}\n---\n{}\n---\n{}\n---\n{}",
        add_2_3.pretty(80),
        add_2_3.pretty(60),
        add_2_3.pretty(40),
        nest(20, line() & add_2_3.format()).pretty(60),
        add_2_3.pretty(20)
    );
}
