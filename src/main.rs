use std::collections::HashMap;

struct RadixTree {}

#[derive(Debug)]
pub struct RadixNode<'a, T> {
    nodes: HashMap<&'a str, Box<RadixNode<'a, T>>>,
    val: Option<T>,
}

impl<'a, T> RadixNode<'a, T> {
    fn new(val: Option<T>) -> RadixNode<'a, T> {
        RadixNode {
            nodes: HashMap::new(),
            val,
        }
    }

    pub fn add(&mut self, path: &'a str, val: Option<T>) -> Result<(), ()> {
        let mut split_string = "";
        if self.nodes.is_empty() {
            self.nodes.insert(path, Box::new(RadixNode::new(val)));
            return Ok(());
        }

        if path.is_empty() {
            self.val = val;
            return Ok(());
        }

        for (key, node) in &mut self.nodes {
            // key - path
            // equal -> check val none -> if true add
            // not equal -> check starts with
            //  -> if true add remain str
            //  -> add
            if &path == key {
                match self.val {
                    Some(_) => return Err(()),
                    None => {
                        self.val = val;
                        return Ok(());
                    }
                }
            }
            if path.starts_with(key) {
                return node.add(&path[key.len() - 1..], val);
            }
            if key.starts_with(path) {
                let (new_key, second_key) = key.split_at(path.len() - 1);
                let mut main_node = RadixNode::new(None);
    
                main_node.add(second_key, node.val);
                main_node.add("", val);
    
                self.nodes.insert(new_key, Box::new(main_node));
    
                return Ok(());
                split_string = key;
                break;
            }
        }

        // if !split_string.is_empty() {
        //     let (key, node) self.nodes.get(split_string);
        //     let (new_key, second_key) = key.split_at(path.len() - 1);
        //     let mut main_node = RadixNode::new(None);

        //     main_node.add(second_key, node.val);
        //     main_node.add("", val);

        //     self.nodes.insert(new_key, Box::new(main_node));

        //     return Ok(());
        // }

        self.nodes.insert(path, Box::new(RadixNode::new(val)));
        return Ok(());
    }

    pub fn get(&self, path: &'a str) -> Option<&T> {
        if self.nodes.is_empty() || path.is_empty() {
            return None;
        }

        for (key, node) in &self.nodes {
            if key == &path {
                if let Some(val) = &node.val {
                    return Some(val);
                }

                return None;
            }
            if path.starts_with(key) {
                return node.get(&path[key.len() - 1..]);
            }
        }

        return None;
    }
}

impl<'a, T> Default for RadixNode<'a, T> {
    fn default() -> Self {
        Self::new(None)
    }
}

fn main() {
    println!("Hello, world!");
}
