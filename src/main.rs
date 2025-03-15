use crate::trie::Trie;

mod node;
mod trie;

fn main() {
    let mut trie = Trie::new();
    trie.add("Test", 10);
    trie.add("Test2", 20);
    trie.add("Test3", 30);
    trie.add("TT", 5);
    trie.add("Testt", 22);

    let values = trie.search("Test", 10);

    for value in values {
        println!("Value of node: {}, priority: {}", value.full_value, value.priority);
    }
}
