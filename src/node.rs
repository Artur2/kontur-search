use std::collections::HashMap;

#[derive(Debug)]
pub struct Node {
    pub key: char,
    pub priority: i64,
    pub full_value: String,
    pub parent: Option<Box<Node>>,
    pub child_nodes: HashMap<char, Box<Node>>,
    pub transitional_nodes: Vec<Box<Node>>,
}

impl Default for Node {
    fn default() -> Self {
        Node {
            key: char::default(),
            parent: None,
            priority: 0,
            full_value: String::default(),
            child_nodes: HashMap::new(),
            transitional_nodes: Vec::new(),
        }
    }
}
