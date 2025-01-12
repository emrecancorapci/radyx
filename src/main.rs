use std::{collections::HashMap, fmt::Debug};

struct RadixTree {}

#[derive(Debug)]
pub struct RadixNode<'a, T: Debug> {
    nodes: HashMap<&'a str, Box<RadixNode<'a, T>>>,
    val: Option<T>,
}

impl<'a, T: Debug> RadixNode<'a, T> {
    fn new(val: Option<T>) -> RadixNode<'a, T> {
        RadixNode {
            nodes: HashMap::new(),
            val,
        }
    }

    pub fn add(&mut self, path: &'a str, val: Option<T>) -> Result<(), String> {
        let mut split_string = "";
        if self.nodes.is_empty() {
            self.nodes.insert(path, Box::new(RadixNode::new(val)));
            return Ok(());
        } else if path.is_empty() {
            self.val = val;
            return Ok(());
        }

        for (key, node) in &mut self.nodes {
            if path.len() > key.len() {
                if path.starts_with(key) {
                    return node.add(&path[key.len()..], val);
                }

                for (index, ch) in key.chars().enumerate(){
                    
                }
            } else if path.len() < key.len() {
                if key.starts_with(path) {
                    split_string = key;
                }
                break;
            } else if &path == key {
                println!("THIS {} {}", path, key);
                match node.val {
                    Some(ref old_val) => return Err(format!("Value is not None, {:?}", old_val)),
                    None => {
                        node.set_value(val);
                        return Ok(());
                    }
                }
            }
        }

        if !split_string.is_empty() {
            let (base_key, splitted_key) = split_string.split_at(path.len());
            let _ = self.split_node(base_key, splitted_key);

            println!("Splitted and Add: {} {:?}", path, &val);
            return self.add(path, val);
        }

        self.nodes.insert(path, Box::new(RadixNode::new(val)));
        return Ok(());
    }

    fn split_node(&mut self, base_key: &'a str, splitted_key: &'a str) -> Result<(), String> {
        let node = self.nodes.remove(format!("{base_key}{splitted_key}").as_str()).unwrap();

        let mut main_node = RadixNode::new(None);

        main_node.add(splitted_key, node.val)?;

        self.nodes.insert(base_key, Box::new(main_node));
        return Ok(());
    }

    fn set_value(&mut self, val: Option<T>) {
        self.val = val;
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
                return node.get(&path[key.len()..]);
            }
        }
        return None;
    }
}

impl<'a, T: Debug> Default for RadixNode<'a, T> {
    fn default() -> Self {
        Self::new(None)
    }
}

fn main() {
    let mut node: RadixNode<'_, String> = RadixNode::default();

    node.add("/home", Some(String::from("Home")));
    let val = node.get("/home");

    println!("{}", val.unwrap())
}
