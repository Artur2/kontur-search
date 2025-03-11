use crate::node::Node;

#[derive(Debug, Default)]
pub struct Trie {
    pub root: Box<Node>,
}

impl Trie {
    pub fn new() -> Trie {
        Trie {
            root: Box::new(Node::default()),
        }
    }
}
