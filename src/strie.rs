use std::collections::HashMap;

#[derive(Default, Clone)]
pub struct SNode {
    pub childs: Box<HashMap<u8, SNode>>,
    pub end_of_word: bool,
    pub full_value: String,
}

impl SNode {
    pub fn new(key: u8) -> SNode {
        SNode {
            childs: Box::new(HashMap::new()),
            end_of_word: false,
            full_value: String::default(),
        }
    }
}

pub struct STrie {
    pub root: SNode,
}

impl STrie {
    pub fn new() -> STrie {
        STrie {
            root: SNode::default(),
        }
    }

    pub fn add(&mut self, value: &str) {
        let mut node = &mut self.root;
        let chars = value.chars();

        for char in chars {
            let char_n = char as u8;
            node = node.childs.entry(char_n).or_default();
        }

        node.end_of_word = true;
        node.full_value = String::from(value);
    }

    pub fn search(&mut self, value: &str) -> Vec<SNode> {
        let mut results = vec![];
        let mut node = &mut self.root;
        let mut chars = value.chars();

        let first_char = chars.nth(0).unwrap() as u8;
        if node.childs.contains_key(&first_char) {
            let mut found_node = node.childs.get_mut(&first_char).unwrap();
            let another_slice = STrie::find_another_words(&mut found_node, value, 1);
            another_slice.iter().for_each(|another_word| {
                results.push(another_word.clone());
            });
        }

        results
    }

    fn find_another_words(node: &SNode, full_value: &str, index: u32) -> Vec<SNode> {
        let mut results = vec![];
        if node.end_of_word {
            results.push(node.clone());
        }

        if node.childs.len() == 0 {
            return results;
        }

        if index + 1 > full_value.len() as u32 {
            STrie::stack_base_search(node)
                .iter()
                .for_each(|another_word| results.push(another_word.clone()));
            return results;
        }

        let searching_char = full_value.chars().nth(index as usize).unwrap();
        let searching_char_as_usize = full_value.as_bytes().get(index as usize).unwrap();

        if node.childs.contains_key(searching_char_as_usize) {
            let node = &node.childs.get(&searching_char_as_usize).unwrap();
            Self::find_another_words(&node, full_value, index + 1)
                .iter()
                .for_each(|another_word| {
                    results.push(another_word.clone());
                });
        }

        results
    }

    fn stack_base_search(node: &SNode) -> Vec<SNode> {
        let mut rolling_nodes = vec![];
        let mut results = vec![];
        rolling_nodes.push(node);

        while !rolling_nodes.is_empty() {
            let rolling_node = rolling_nodes.pop().unwrap();

            if rolling_node.end_of_word {
                results.push(rolling_node.clone());
                continue;
            }

            for child in rolling_node.childs.values() {
                rolling_nodes.push(child);
            }
        }

        results
    }
}

#[cfg(test)]
pub mod test {
    use super::*;

    #[test]
    pub fn should_add_word() {
        let mut trie = STrie::new();
        trie.add("hello");
    }

    #[test]
    pub fn should_search() {
        let mut trie = STrie::new();
        trie.add("hello");
        trie.add("hello2");

        let results = trie.search("hello");
        assert_eq!(results.len(), 2);
    }

    #[test]
    pub fn should_search_different_words() {
        let mut trie = STrie::new();
        trie.add("hello");
        trie.add("hello2");

        trie.add("world");
        trie.add("world2");

        let mut results = trie.search("hello");
        assert_eq!(results.len(), 2);

        results = trie.search("world");
        assert_eq!(results.len(), 2);
    }
}
