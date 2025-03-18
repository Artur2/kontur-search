use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

pub struct Node {
    pub key: char,
    pub parent: Option<Rc<RefCell<Node>>>,
    pub nodes: HashMap<char, Rc<RefCell<Node>>>,
    pub transitional_nodes: Vec<Rc<TransitionalNode>>
}

#[derive(Default)]
pub struct TransitionalNode {
    pub full_value: String,
    pub priority: i64,
}

impl Node {
    pub fn new(key: char) -> Node {
        let mut default = Node::default();
        default.key = key;

        default
    }
}

impl TransitionalNode {
    pub fn new(priority: i64, full_value: String) -> TransitionalNode {
        TransitionalNode {
            priority,
            full_value
        }
    }
}

impl Default for Node {
    fn default() -> Self {
        Node {
            key: char::default(),
            nodes: HashMap::new(),
            transitional_nodes: Vec::new(),
            parent: None
        }
    }
}
