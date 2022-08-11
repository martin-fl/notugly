notugly
=============

Simple and generic pretty-printing library, heavily based on
[A prettier printer](https://homepages.inf.ed.ac.uk/wadler/papers/prettier/prettier.pdf).

## How to use

To define pretty-printing for a type, implement the `Format` trait using the various utility functions:
- `nil` for a null element
- `text` to create a string into a document
- `nest` to indent a document
- `cat` to concatenate two documents
- `group` and `group_with` to add the flattened layout as an alternative.
- `fold`, `spread`, `stack` and `fill` to collapse a list of documents in various ways


To make it easier to define a structure, some operators are defined:
- `x & y == cat(x,y)`
- `x + y == x & text(" ") & y`
- `x / y == x & line() & y`
- `x * y == x & group(line) & y`

See the [examples/](examples) directory for fully-featured examples.

## Example : S-Expressions

```rust
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
fn main() {
    let big_eq = sexpr!(add (mul 2 6) (div (mul 4 (mul 3 2 1)) (add 1 (sub 3 (add 1 1)))));

    println!("{}", big_eq.pretty(40));
}
```