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
        let mut split_string = "";

        if self.nodes.is_empty() {
            self.nodes.insert(path, Box::new(RadixNode::new(val)));
            return Ok(());
        } else if path.is_empty() {
            self.set_value(Some(val));
            return Ok(());
        }

        for (key, node) in &mut self.nodes {
            if key.is_empty() {
                continue;
            }

            let splitting_index = find_common_index(path, key);

            if splitting_index == 0 {
                continue;
            }

            if splitting_index == key.len() {
                if path.starts_with(key) {
                    return node.insert(&path[key.len()..], val);
                }

                let (base_key, splitted_key) = path.split_at(splitting_index);
                let _ = self.split_node(base_key, splitted_key);

                return self.insert(path, val);
            }

            if splitting_index == path.len() {
                if key.starts_with(path) {
                    split_string = key;
                    break;
                }

                let (base_key, splitted_key) = key.split_at(splitting_index);
                let _ = self.split_node(base_key, splitted_key);

                return self.insert(path, val);
            }

            if &path == key {
                match node.val {
                    Some(ref old_val) => return Err(format!("Value is not None, {:?}", old_val)),
                    None => {
                        node.set_value(Some(val));
                        return Ok(());
                    }
                }
            }

            let (base_key, splitted_key) = key.split_at(splitting_index);
            let _ = self.split_node(base_key, splitted_key);

            return self.insert(path, val);
        }

        if !split_string.is_empty() {
            let (base_key, splitted_key) = split_string.split_at(path.len());
            let _ = self.split_node(base_key, splitted_key);

            return self.insert(path, val);
        }

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
                return node.get(&path[key.len()..]);
            }
        }
        return None;
    }

    fn split_node(&mut self, base_key: &'a str, splitted_key: &'a str) -> Result<(), String> {
        let node = self
            .nodes
            .remove(format!("{base_key}{splitted_key}").as_str())
            .unwrap();

        let mut main_node: RadixNode<'_, T> = RadixNode::new_empty();

        main_node.insert(splitted_key, node.val.unwrap())?;

        self.nodes.insert(base_key, Box::new(main_node));
        return Ok(());
    }

    fn set_value(&mut self, val: Option<T>) {
        self.val = val;
    }

}

impl<'a, T: Debug> Default for RadixNode<'a, T> {
    fn default() -> Self {
        Self::new_empty()
    }
}

pub fn find_common_index(s1: &str, s2: &str) -> usize {
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
