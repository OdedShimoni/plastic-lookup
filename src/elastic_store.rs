use serde_json::{Value, Map};

use crate::elastic_node::ElasticNode;

#[derive(Debug)]
pub struct ElasticStore {
    pub data: Map<String, Value>,
    pub nodes: ElasticNode,
}

impl ElasticStore {
    pub fn new() -> Self {
        ElasticStore {
            data: Map::new(),
            nodes: ElasticNode::new(None)
        }
    }
    pub fn add_word(&mut self, word: String, reference: String, data: Value) {
        self.data.insert(reference.clone(), data);
        let mut i = &mut self.nodes;
        let mut it = word.chars().peekable();
        while let Some(c) = it.next()  {
            if it.peek().is_none() {
                // last iteration
                i = i.add(c, Some(reference.clone()));
            } else {
                i = i.add(c, None);
            }
        }
    }
}