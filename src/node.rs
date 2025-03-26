use fxhash::FxHashMap;
use std::cell::RefCell;
use std::rc::Rc;

pub struct Node {
    pub nodes: Box<FxHashMap<u8, Rc<RefCell<Node>>>>,
    pub transitional_node: Option<TransitionalNode>
}

#[derive(Default)]
pub struct TransitionalNode {
    pub full_value: Vec<u8>,
    pub priority: i64,
}

impl Node {
    pub fn new() -> Node {
        let mut default = Node::default();
        default
    }
}

impl TransitionalNode {
    pub fn new(priority: i64, full_value: &str) -> TransitionalNode {

        let mut bytes = vec![];
        full_value.chars().for_each(|c| {
            bytes.push(c as u8);
        });

        TransitionalNode {
            priority,
            full_value: bytes
        }
    }

    pub fn full_value_as_string(&self) -> String {
        String::from_utf8(self.full_value.clone()).unwrap()
    }
}

impl Default for Node {
    fn default() -> Self {
        Node {
            nodes: Box::new(FxHashMap::default()),
            transitional_node: None
        }
    }
}