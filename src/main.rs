use crate::trie::Trie;
use std::fmt::Debug;

mod node;
mod trie;

#[allow(unused_imports, dead_code)]

fn main() {
    let mut trie = Trie::new();
    trie.add("Test", 10);
    trie.add("Test2", 20);

    let values = trie.search("Test", 2);

    for value in values {
        println!("Value of node: {}", value.full_value);
    }
}
