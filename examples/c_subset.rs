use notugly::*;

pub mod ast {
    pub(crate) struct Ident(pub(crate) String);

    pub(crate) struct CStruct {
        pub(crate) name: Ident,
        pub(crate) fields: Vec<(CType, Ident)>,
    }
    pub(crate) struct CEnum {
        pub(crate) name: Ident,
        pub(crate) variants: Vec<Ident>,
    }

    pub(crate) enum CType {
        Struct(Ident),
        Enum(Ident),
        Int,
        Char,
        Ptr(Box<CType>),
    }

    pub(crate) struct CFunction {
        pub(crate) name: Ident,
        pub(crate) ret: CType,
        pub(crate) params: Vec<(CType, Ident)>,
        pub(crate) body: Vec<Stmt>,
    }

    pub(crate) enum Stmt {
        Decl(CDecl),
        Assignment(Ident, CExpr),
        Expr(CExpr),
        Return(CExpr),
    }

    pub(crate) enum CExpr {
        IntLit(i32),
        StrLit(String),
        IdentLit(Ident),
        Call(Ident, Vec<CExpr>),
    }

    pub(crate) enum CDecl {
        StructDecl(CStruct),
        EnumDecl(CEnum),
        VarDecl(CType, Ident),
        FunctionDecl(CFunction),
    }
}

pub mod fmt {
    use notugly::*;

    use crate::ast::*;

    fn sep(xs: &[impl Format], sep: &str) -> Document {
        fold(xs, |lhs, rhs| lhs & text(sep) + rhs)
    }

    fn paren(x: Document) -> Document {
        group_with("", group(text("(") & x / text(")")))
    }

    impl Format for Ident {
        fn format(&self) -> Document {
            text(&self.0)
        }
    }

    impl Format for CStruct {
        fn format(&self) -> Document {
            let fields = self
                .fields
                .iter()
                .map(|(ty, id)| ty.format() + id.format() & text(";"))
                .collect::<Vec<_>>();
            group(text("struct") + self.name.format() + bracket(4, "{", stack(&fields), "}"))
        }
    }

    impl Format for CEnum {
        fn format(&self) -> Document {
            group(
                text("enum") + self.name.format() + bracket(4, "{", sep(&self.variants, ","), "}"),
            )
        }
    }

    impl Format for CType {
        fn format(&self) -> Document {
            match self {
                CType::Struct(id) => text("struct") + id.format(),
                CType::Enum(id) => text("enum") + id.format(),
                CType::Int => text("int"),
                CType::Char => text("char"),
                CType::Ptr(ty) => ty.format() & text("*"),
            }
        }
    }

    impl Format for CFunction {
        fn format(&self) -> Document {
            let params = self
                .params
                .iter()
                .map(|(ty, id)| ty.format() + id.format() & text(","))
                .collect::<Vec<_>>();

            self.ret.format() + self.name.format()
                & paren(sep(&params, ",")) + bracket(4, "{", stack(&self.body), "}")
        }
    }

    impl Format for Stmt {
        fn format(&self) -> Document {
            match self {
                Stmt::Decl(d) => d.format(),
                Stmt::Assignment(id, e) => id.format() + text("=") + e.format() & text(";"),
                Stmt::Expr(e) => e.format() & text(";"),
                Stmt::Return(r) => text("return") + r.format() & text(";"),
            }
        }
    }

    impl Format for CExpr {
        fn format(&self) -> Document {
            match self {
                CExpr::IntLit(i) => text(format!("{i}")),
                CExpr::StrLit(s) => text(format!("\"{s}\"")),
                CExpr::IdentLit(i) => i.format(),
                CExpr::Call(f, args) => f.format() & paren(sep(args, ",")),
            }
        }
    }

    impl Format for CDecl {
        fn format(&self) -> Document {
            match self {
                CDecl::StructDecl(s) => s.format() & text(";"),
                CDecl::EnumDecl(e) => e.format() & text(";"),
                CDecl::VarDecl(ty, id) => ty.format() + id.format() & text(";"),
                CDecl::FunctionDecl(f) => f.format(),
            }
        }
    }
}

fn main() {
    use ast::*;

    // Not necessarily correct program but you get it
    let c_prog = &[
        CDecl::StructDecl(CStruct {
            name: Ident("point".into()),
            fields: vec![
                (CType::Int, Ident("x".into())),
                (CType::Int, Ident("y".into())),
            ],
        }),
        CDecl::EnumDecl(CEnum {
            name: Ident("state".into()),
            variants: vec![
                Ident("left".into()),
                Ident("right".into()),
                Ident("up".into()),
                Ident("down".into()),
                Ident("forward".into()),
                Ident("backward".into()),
                Ident("random".into()),
            ],
        }),
        CDecl::FunctionDecl(CFunction {
            name: Ident("main".into()),
            ret: CType::Int,
            params: vec![],
            body: vec![
                Stmt::Decl(CDecl::VarDecl(CType::Int, Ident("n".into()))),
                Stmt::Decl(CDecl::VarDecl(
                    CType::Ptr(Box::new(CType::Char)),
                    Ident("fmt".into()),
                )),
                Stmt::Assignment(Ident("n".into()), CExpr::IntLit(32)),
                Stmt::Assignment(
                    Ident("fmt".into()),
                    CExpr::StrLit("Hello world n°%d!".into()),
                ),
                Stmt::Expr(CExpr::Call(
                    Ident("printf".into()),
                    vec![
                        CExpr::IdentLit(Ident("fmt".into())),
                        CExpr::IdentLit(Ident("n".into())),
                    ],
                )),
                Stmt::Decl(CDecl::VarDecl(
                    CType::Struct(Ident("point".into())),
                    Ident("p".into()),
                )),
                Stmt::Decl(CDecl::VarDecl(
                    CType::Enum(Ident("state".into())),
                    Ident("s".into()),
                )),
                Stmt::Return(CExpr::IntLit(0)),
            ],
        }),
    ];

    println!("{}", stack(c_prog).pretty(60));
}
