use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

pub struct Node {
    pub key: u8,
    pub parent: Option<Rc<RefCell<Node>>>,
    pub nodes: HashMap<u8, Rc<RefCell<Node>>>,
    pub transitional_node: TransitionalNode
}

#[derive(Default)]
pub struct TransitionalNode {
    pub full_value: String,
    pub priority: i64,
}

impl Node {
    pub fn new(key: u8) -> Node {
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
            key: u8::default(),
            nodes: HashMap::new(),
            transitional_node: TransitionalNode::default(),
            parent: None
        }
    }
}
