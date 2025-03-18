use crate::node::*;
use std::{
    cell::{RefCell, RefMut},
    rc::Rc,
};

#[derive(Default)]
pub struct Trie {
    root: Rc<RefCell<Node>>,
}

impl Trie {
    pub fn new() -> Trie {
        Trie {
            root: Rc::default(),
        }
    }

    pub fn add(&mut self, value: &str, priority: i64) {
        let length: usize = value.len();
        let value_as_bytes = value.as_bytes();
        let transitional_node = TransitionalNode::new(priority, String::from(value));
        let transitional_node_rc = Rc::from(transitional_node);

        let root_rc: Rc<RefCell<Node>> = Rc::clone(&self.root);
        let mut root_node_mut = self.root.borrow_mut();
        let mut rolling_node_rc = Rc::default();
        let mut is_next_level = false;

        for i in 0..length {
            let symbol = value_as_bytes[i] as char;

            if !is_next_level {
                // Ничего не задано с рута
                rolling_node_rc = Trie::create_or_rollout_another_symbol(
                    &symbol,
                    &transitional_node_rc,
                    &mut root_node_mut,
                    root_rc.clone(),
                );
                is_next_level = true;
            } else {
                // Уже находимся в на уровне ниже рута
                let new_rolling_node_rc;
                match rolling_node_rc.try_borrow_mut() {
                    Ok(mut inner_node_mut) => {
                        new_rolling_node_rc = Trie::create_or_rollout_another_symbol(
                            &symbol,
                            &transitional_node_rc,
                            &mut inner_node_mut,
                            rolling_node_rc.clone(),
                        );
                    }
                    Err(_) => {
                        panic!("Какого хрена мы не можем заимствовать")
                    }
                }

                rolling_node_rc = new_rolling_node_rc;
            }
        }
    }

    pub fn search(&self, value: &str, max_results: i32) -> Vec<Rc<TransitionalNode>> {
        let mut results = Vec::new();
        let value_length: usize = value.len();

        let value_as_bytes = value.as_bytes();
        let root_node = self.root.borrow();

        let mut is_next_level = false;
        let mut rolling_node_rc = Rc::default();
        let mut is_passed_whole_value = false;

        for i in 0..value_length {
            let symbol = value_as_bytes[i] as char;
            if i == value_length - 1 {
                is_passed_whole_value = true;
            }

            if !is_next_level && root_node.nodes.contains_key(&symbol) {
                rolling_node_rc = root_node.nodes[&symbol].clone();
                is_next_level = true;
            } else {
                let mut new_rolling_node_rc = Rc::default();
                let mut found_rolling_node = false;
                match rolling_node_rc.try_borrow_mut() {
                    Ok(inner_node_mut) => {
                        if inner_node_mut.nodes.contains_key(&symbol) {
                            new_rolling_node_rc = inner_node_mut.nodes[&symbol].clone();
                            found_rolling_node = true;
                        }
                    }
                    Err(_) => {
                        panic!("Cannot borrow value?");
                    }
                }

                if found_rolling_node {
                    rolling_node_rc = new_rolling_node_rc;
                } else {
                    is_passed_whole_value = false;
                    break;
                }
            }
        }

        if is_passed_whole_value == false {
            return results;
        }

        let mut nodes_without_childs = Trie::find_nodes_without_childs(Rc::clone(&rolling_node_rc));

        nodes_without_childs.sort_by(|a, b| {
            let b_transitional_node = b.borrow().transitional_nodes[0].clone();
            let a_transitional_node = a.borrow().transitional_nodes[0].clone();

            return b_transitional_node
                .priority
                .cmp(&a_transitional_node.priority);
        });

        nodes_without_childs
            .iter()
            .map(|v| v.borrow().transitional_nodes[0].clone())
            .take(max_results as usize)
            .collect()
    }

    fn find_nodes_without_childs(node: Rc<RefCell<Node>>) -> Vec<Rc<RefCell<Node>>> {
        let mut stack = vec![];
        let mut nodes_without_childs: Vec<Rc<RefCell<Node>>> = vec![];

        stack.push(node);

        while !stack.is_empty() {
            let inner_node_rc = stack.pop().unwrap();
            let inner_node = inner_node_rc.borrow();

            if inner_node.nodes.len() == 0 {
                nodes_without_childs.push(inner_node_rc.clone());
                continue;
            }

            inner_node.nodes.iter().for_each(|kvp| {
                stack.push(kvp.1.clone());
            });
        }
        nodes_without_childs
    }

    fn create_node_with_transition(
        char: &char,
        transitional_node: &Rc<TransitionalNode>,
        parent: Rc<RefCell<Node>>,
    ) -> Rc<RefCell<Node>> {
        let mut node = Node::new(char.clone());
        node.transitional_nodes.push(transitional_node.clone());
        node.parent = Some(parent);
        let new_node_cell = RefCell::new(node); // mutable mem location
        let new_node_rc = Rc::from(new_node_cell); // reference to location
        new_node_rc
    }

    fn create_or_rollout_another_symbol(
        symbol: &char,
        transitional_node: &Rc<TransitionalNode>,
        node: &mut RefMut<Node>,
        parent: Rc<RefCell<Node>>,
    ) -> Rc<RefCell<Node>> {
        if !node.nodes.contains_key(symbol) {
            let new_node_rc = Trie::create_node_with_transition(&symbol, transitional_node, parent);
            let cloned_rc_from_new_node_rc = Rc::clone(&new_node_rc);
            node.nodes.insert(symbol.clone(), new_node_rc);
            cloned_rc_from_new_node_rc
        } else {
            let existing_rc = Rc::clone(&node.nodes[symbol]);
            let mut existing_node = existing_rc.borrow_mut();

            existing_node
                .transitional_nodes
                .push(transitional_node.clone());

            existing_rc.clone()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

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

        assert_eq!(result[0].priority, 3);
        assert_eq!(result[1].priority, 2);
        assert_eq!(result[2].priority, 1);
    }

    #[test]
    pub fn find_different_values() {
        let mut trie = Trie::new();
        trie.add("a", 1);
        trie.add("b", 1);

        let first_result = trie.search("a", 1);
        let second_result = trie.search("b", 1);

        assert_eq!(first_result.len(), 1);
        assert_eq!(second_result.len(), 1);
        assert_eq!(first_result[0].priority, 1);
        assert_eq!(second_result[0].priority, 1);
    }

    #[test]
    pub fn not_find_value_with_empty() {
        let trie = Trie::new();
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
}
