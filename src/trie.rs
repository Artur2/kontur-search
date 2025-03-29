use crate::node::*;

#[derive(Default)]
pub struct Trie {
    root: Node,
}

impl Trie {
    pub fn new() -> Trie {
        Trie {
            root: Node::default(),
        }
    }

    pub fn add(&mut self, value: &str, priority: i64) {
        let length: usize = value.len();
        let value_as_bytes = value.as_bytes();
        let transitional_node = TransitionalNode::new(priority, &value);
        let mut root_node_mut = &mut self.root;

        for i in 0..length {
            let symbol = value_as_bytes[i];
            root_node_mut = Self::create_or_rollout_node(&symbol, root_node_mut);
        }

        root_node_mut.transitional_node = Some(transitional_node);
    }

    pub fn contains(&self, value: &str) -> bool {
        let result = self.rollout_node(&value);
        match result {
            None => false,
            Some(_) => true,
        }
    }

    pub fn search(&self, value: &str, max_results: i32) -> Vec<&Node> {
        let mut results = Vec::new();
        let found = self.rollout_node(value);

        if found.is_none() {
            return results;
        }

        let found = found.unwrap();

        let mut nodes_without_childs = Self::find_nodes_without_childs(found);
        nodes_without_childs.sort_by(|a, b| {
            let node_a = a.transitional_node.as_ref().unwrap();
            let node_b = b.transitional_node.as_ref().unwrap();

            node_b.priority.cmp(&node_a.priority)
        });

        let mut counter = 0;
        nodes_without_childs.iter().for_each(|node| {
            if counter == max_results + 1 {
                return;
            }

            results.push(*node);
            counter += 1;
        });

        results
    }

    fn rollout_node(&self, value: &str) -> Option<&Node> {
        let value_length: usize = value.len();
        let value_as_bytes: &[u8] = value.as_bytes();
        let mut root_node = &self.root;
        let mut is_passed_whole_value = false;

        for i in 0..value_length {
            let symbol = value_as_bytes[i];
            if i == value_length - 1 {
                is_passed_whole_value = true;
            }

            if root_node.nodes.contains_key(&symbol) {
                root_node = &root_node.nodes[&symbol];
            } else {
                is_passed_whole_value = false;
                break;
            }
        }

        if is_passed_whole_value {
            Some(&root_node)
        } else {
            None
        }
    }

    fn create_or_rollout_node<'a>(symbol: &u8, node: &'a mut Node) -> &'a mut Node {
        if node.nodes.contains_key(&symbol) {
            node.nodes.get_mut(&symbol).unwrap()
        } else {
            node.nodes.entry(symbol.clone()).or_insert(Node::new())
        }
    }

    fn find_nodes_without_childs(node: &Node) -> Vec<&Node> {
        let mut stack = vec![];
        let mut nodes_without_childs: Vec<&Node> = vec![];

        stack.push(node);

        while !stack.is_empty() {
            let inner_node = stack.pop().unwrap();

            if inner_node.nodes.len() == 0 {
                nodes_without_childs.push(inner_node);
                continue;
            }

            inner_node.nodes.iter().for_each(|kvp| {
                stack.push(kvp.1);
            });
        }

        nodes_without_childs
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn should_correct_add() {
        let mut trie = Trie::new();

        trie.add("test test", 2);
    }

    #[test]
    pub fn after_add_is_available() {
        let mut trie = Trie::new();
        trie.add("a", 1);
        let result = trie.search("a", 1);
        assert_eq!(result.len(), 1);
    }

    #[test]
    pub fn respect_count_of_search() {
        let mut trie = Trie::new();
        trie.add("ac", 1);
        trie.add("ab", 2);
        trie.add("c", 3);
        let result = trie.search("a", 2);

        assert_eq!(result.len(), 2);
    }

    #[test]
    pub fn respect_priority_with_max_results() {
        let mut trie = Trie::new();
        trie.add("aaa", 1);
        trie.add("aa3", 2);
        trie.add("aahhh", 3);

        let result = trie.search("aa", 3);

        let value_0 = result[0];
        let value_1 = result[1];
        let value_2 = result[2];

        assert_eq!(value_0.transitional_node.as_ref().unwrap().priority, 3);
        assert_eq!(value_1.transitional_node.as_ref().unwrap().priority, 2);
        assert_eq!(value_2.transitional_node.as_ref().unwrap().priority, 1);
    }

    #[test]
    pub fn find_different_values() {
        let mut trie = Trie::new();
        trie.add("a", 1);
        trie.add("b", 1);

        let first_result = trie.search("a", 1);
        let second_result = trie.search("b", 1);

        let borrowed_0 = first_result[0];
        let borrowed_1 = second_result[0];

        assert_eq!(first_result.len(), 1);
        assert_eq!(second_result.len(), 1);
        assert_eq!(borrowed_0.transitional_node.as_ref().unwrap().priority, 1);
        assert_eq!(borrowed_1.transitional_node.as_ref().unwrap().priority, 1);
    }

    #[test]
    pub fn not_find_value_with_empty() {
        let mut trie = Trie::new();
        let result = trie.search("aaa", 1);

        assert_eq!(result.len(), 0);
    }

    #[test]
    pub fn not_find_value_with_seeded_trie() {
        let mut trie = Trie::new();
        trie.add("dead beef", 1);
        let result = trie.search("dead beef1", 1);
        assert_eq!(result.len(), 0);
    }

    #[test]
    pub fn not_find_different_value_with_seeded_trie() {
        let mut trie = Trie::new();
        trie.add("dead beef", 1);
        let result = trie.search("deface", 1);
        assert_eq!(result.len(), 0);
    }

    #[test]
    pub fn search_correct_ignore_if_word_not_full_is_exist() {
        let mut trie = Trie::new();
        trie.add("passoublie", 1);

        let result = trie.search("passout", 1);
        assert_eq!(result.len(), 0);
    }

    #[test]
    pub fn search_should_not_overlap() {
        let mut trie = Trie::new();
        trie.add("lapse", 1);

        let result = trie.search("lapsed", 3);
        assert_eq!(0, result.len());
    }

    #[test]
    pub fn search_should_not_find_value() {
        let mut trie = Trie::new();
        trie.add("lapse", 1);
        trie.add("lap", 2);

        let result = trie.search("lapsed", 3);
        assert_eq!(0, result.len());
    }

    #[test]
    pub fn contain_should_work_properly() {
        let mut trie = Trie::new();
        trie.add("lapse", 1);
        trie.add("lap", 2);

        let result = trie.contains("lap");
        assert_eq!(true, result);

        let result = trie.contains("lapse");
        assert_eq!(true, result);
    }
}
