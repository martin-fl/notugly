use notugly::*;
enum Tree {
    Node(String, Vec<Tree>),
    Leaf(String),
}

impl Format for Tree {
    fn format(&self) -> Formatted {
        match self {
            Tree::Node(v, t) => text(v) + bracket(4, "[", spread(t), "]"),
            Tree::Leaf(v) => text(v),
        }
    }
}

fn main() {
    let tree = Tree::Node(
        "aaa".into(),
        vec![
            Tree::Node(
                "bbbbb".into(),
                vec![Tree::Leaf("ccc".into()), Tree::Leaf("dd".into())],
            ),
            Tree::Leaf("eee".into()),
            Tree::Node(
                "ffff".into(),
                vec![
                    Tree::Leaf("gg".into()),
                    Tree::Leaf("hhh".into()),
                    Tree::Leaf("ii".into()),
                ],
            ),
        ],
    );

    println!("{}", tree.pretty(45));

    let hello =
        group(group(group(group(text("hello") / text("a")) / text("b")) / text("c")) / text("d"));

    println!("{}", hello.clone().pretty(9));
}
