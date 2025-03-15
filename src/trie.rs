use crate::node::*;
use std::{
    cell::{RefCell, RefMut},
    rc::Rc,
};

#[derive(Default)]
pub struct Trie {
    pub root: Rc<RefCell<Node>>,
}

impl Trie {
    pub fn new() -> Trie {
        Trie {
            root: Rc::new(RefCell::new(Node::default())),
        }
    }

    pub fn add(&mut self, value: &str, priority: i64) {
        let length: usize = value.len();
        let value_as_bytes = value.as_bytes();
        let transitional_node = TransitionalNode::new(priority, String::from(value));
        let transitional_node_rc = Rc::from(transitional_node);

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
        let mut value_found = true;
        let mut rolling_node_rc = Rc::default();

        for i in 0..value_length {
            let symbol = value_as_bytes[i] as char;
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
                    value_found = false;
                    break;
                }
            }
        }

        if !value_found {
            return results;
        }

        let mut resulting_node = rolling_node_rc.borrow_mut();
        resulting_node
            .transitional_nodes
            .sort_by(|a, b| b.priority.cmp(&a.priority));

        resulting_node
            .transitional_nodes
            .iter()
            .for_each(|node_rc| {
                if results.len() == max_results as usize {
                    return;
                }
                results.push(node_rc.clone());
            });

        results
    }

    fn create_node_with_transition(
        char: &char,
        transitional_node: &Rc<TransitionalNode>,
    ) -> Rc<RefCell<Node>> {
        let mut node = Node::new(char.clone());
        node.transitional_nodes.push(transitional_node.clone());
        let new_node_cell = RefCell::new(node); // mutable mem location
        let new_node_rc = Rc::from(new_node_cell); // reference to location
        new_node_rc
    }

    fn create_or_rollout_another_symbol(
        symbol: &char,
        transitional_node: &Rc<TransitionalNode>,
        node: &mut RefMut<Node>,
    ) -> Rc<RefCell<Node>> {
        if !node.nodes.contains_key(symbol) {
            let new_node_rc = Trie::create_node_with_transition(&symbol, transitional_node);
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
