use std::{collections::HashMap, fmt::Debug};

#[cfg(test)]
mod tests;

#[derive(Debug)]
pub struct RadixNode<'a, T: Debug> {
    nodes: HashMap<&'a str, Box<RadixNode<'a, T>>>,
    val: Option<T>,
}

impl<'a, T: Debug> RadixNode<'a, T> {
    pub fn new(val: T) -> RadixNode<'a, T> {
        RadixNode {
            nodes: HashMap::new(),
            val: Some(val),
        }
    }

    pub fn new_empty() -> RadixNode<'a, T> {
        RadixNode {
            nodes: HashMap::new(),
            val: None,
        }
    }

    pub fn insert(&mut self, path: &'a str, val: T) -> Result<(), String> {
        self.insert_value(path, Some(val))
    }

    pub fn get(&self, path: &'a str) -> Option<&T> {
        if self.nodes.is_empty() {
            return None;
        }

        for (key, node) in &self.nodes {
            if key == &path {
                if let Some(val) = &node.val {
                    return Some(val);
                }
                return None;
            } else if let Some(stripped) = path.strip_prefix(key) {
                return node.get(stripped);
            }
        }

        None
    }

    fn insert_value(&mut self, path: &'a str, val: Option<T>) -> Result<(), String> {
        if self.nodes.is_empty() {
            if let Some(val) = val {
                self.nodes.insert(path, Box::new(RadixNode::new(val)));
            } else {
                self.nodes.insert(path, Box::new(RadixNode::new_empty()));
            }
            return Ok(());
        } else if path.is_empty() {
            self.val = val;
            return Ok(());
        }

        for (key, node) in &mut self.nodes {
            let splitting_index = match longest_common_prefix(path, key) {
                0 => continue,
                v => v,
            };

            if key == &path {
                node.val = val;
                return Ok(());
            }

            let key_length = key.len();
            let path_length = path.len();

            if key_length == splitting_index && path.starts_with(key) {
                return node.insert_value(&path[key_length..], val);
            }

            let (base_key, splitted_key) = if key_length == splitting_index {
                path.split_at(splitting_index)
            } else if path_length == splitting_index && key.starts_with(path) {
                key.split_at(path_length)
            } else {
                key.split_at(splitting_index)
            };

            self.split_node(base_key, splitted_key)?;
            return self.insert_value(path, val);
        }

        match val {
            Some(val) => self.nodes.insert(path, Box::new(RadixNode::new(val))),
            None => self.nodes.insert(path, Box::new(RadixNode::new_empty())),
        };

        Ok(())
    }

    fn split_node(&mut self, base_key: &'a str, splitted_key: &'a str) -> Result<(), String> {
        let node = self
            .nodes
            .remove(format!("{base_key}{splitted_key}").as_str())
            .unwrap();

        let mut main_node: RadixNode<'_, T> = RadixNode::new_empty();

        main_node.insert_value(splitted_key, node.val)?;
        main_node.fill_node(splitted_key, node.nodes);

        self.nodes.insert(base_key, Box::new(main_node));

        Ok(())
    }

    fn fill_node(
        &mut self,
        splitted_key: &'a str,
        new_nodes: HashMap<&'a str, Box<RadixNode<'a, T>>>,
    ) {
        let new_node = self.nodes.get_mut(splitted_key).unwrap();

        if new_node.nodes.is_empty() {
            new_node.nodes = new_nodes;
        }
    }
}

impl<T: Debug> Default for RadixNode<'_, T> {
    fn default() -> Self {
        Self::new_empty()
    }
}

/// Finds the first differing index between two strings
pub fn longest_common_prefix(s1: &str, s2: &str) -> usize {
    let mut iter1 = s1.chars();
    let mut iter2 = s2.chars();

    for i in 0.. {
        match (iter1.next(), iter2.next()) {
            (Some(ch1), Some(ch2)) if ch1 == ch2 => continue,
            _ => return i,
        }
    }

    0
}
