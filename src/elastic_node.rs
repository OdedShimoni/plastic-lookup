use std::{collections::HashMap};

#[derive(Debug)]
pub struct ElasticNode {
    pub children: HashMap<char, ElasticNode>,
    pub included_reference_key: Option<String>,
}

impl ElasticNode {
    pub fn new(reference: Option<String>) -> Self {
        // say data is kept in a hashmap of json (string => string/array/object/number)
        Self {
            children: HashMap::new(),
            included_reference_key: reference
        }
    }
    pub fn add(&mut self, char: char, reference: Option<String>) -> &mut ElasticNode {
        // consider wrap in Result
        let child = self.children.entry(char)
            .or_insert(
                Self::new(reference.clone())
            );
        child.included_reference_key = reference;
        self.children.get_mut(&char).unwrap()
    }
    pub fn lookup(&self, text: &str) -> Option<&ElasticNode> {
        // TODO clean
        // TODO make function return Option with &String
        let mut current_node_pointer = self;
        let mut found_option: Option<&ElasticNode> = None;
        for c in text.chars() {
            found_option = look(c,  current_node_pointer);
            match found_option {
                Some(found_tree) => {
                    current_node_pointer = found_tree
                }
                None => {
                    return None
                }
            }
        }
        found_option
    }
}

pub fn look(char: char, tree: &ElasticNode) -> Option<&ElasticNode> {
    let next_match_option = tree.children.get(&char);
    next_match_option
}