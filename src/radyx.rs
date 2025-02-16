use std::{collections::HashMap, fmt::Debug};

use crate::utils::longest_common_prefix;

#[derive(Debug)]
pub struct Radyx<'a, T: Debug> {
    nodes: HashMap<&'a str, Box<Radyx<'a, T>>>,
    val: Option<T>,
}

impl<'a, T: Debug> Radyx<'a, T> {
    /// Create a new node
    ///
    /// # Example
    ///
    /// ```
    /// use radyx::Radyx;
    ///
    /// let mut node: Radyx<String> = Radyx::new(String::from("Radyx"));
    /// ```
    pub fn new(val: T) -> Radyx<'a, T> {
        Radyx {
            nodes: HashMap::new(),
            val: Some(val),
        }
    }

    /// Create a new empty node
    ///
    /// # Example
    ///
    /// ```
    /// use radyx::Radyx;
    ///
    /// let mut node: Radyx<String> = Radyx::new_empty();
    /// ```
    pub fn new_empty() -> Radyx<'a, T> {
        Radyx {
            nodes: HashMap::new(),
            val: None,
        }
    }

    /// Insert new value to the node
    ///
    /// # Example
    ///
    /// ```
    /// use radyx::Radyx;
    ///
    /// let mut node: Radyx<String> = Radyx::new_empty();
    /// ```
    pub fn insert(&mut self, path: &'a str, val: T) {
        self.insert_node(path, Some(val));
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
            } else if let Some(stripped) = path.strip_prefix(key) {
                if !stripped.is_empty() && !key.is_empty() {
                    return node.get(stripped);
                }
            }
        }

        None
    }

    fn insert_node(&mut self, path: &'a str, val: Option<T>) {
        if self.nodes.is_empty() {
            if let Some(val) = val {
                self.nodes.insert(path, Box::new(Radyx::new(val)));
            } else {
                self.nodes.insert(path, Box::new(Radyx::new_empty()));
            }
            return;
        } else if path.is_empty() {
            self.val = val;
            return;
        }

        for (key, node) in &mut self.nodes {
            let splitting_index = match longest_common_prefix(path, key) {
                0 => continue,
                v => v,
            };

            if key == &path {
                node.val = val;
                return;
            }

            let key_length = key.len();
            let path_length = path.len();

            if key_length == splitting_index && path.starts_with(key) {
                return node.insert_node(&path[key_length..], val);
            }

            let (base_key, splitted_key) = if key_length == splitting_index {
                path.split_at(splitting_index)
            } else if path_length == splitting_index && key.starts_with(path) {
                key.split_at(path_length)
            } else {
                key.split_at(splitting_index)
            };

            self.split_node(base_key, splitted_key);
            return self.insert_node(path, val);
        }

        match val {
            Some(val) => self.nodes.insert(path, Box::new(Radyx::new(val))),
            None => self.nodes.insert(path, Box::new(Radyx::new_empty())),
        };
    }

    fn split_node(&mut self, base_key: &'a str, splitted_key: &'a str) {
        let node = self
            .nodes
            .remove(format!("{base_key}{splitted_key}").as_str())
            .unwrap();

        let mut main_node: Radyx<'_, T> = Radyx::new_empty();

        main_node.insert_node(splitted_key, node.val);
        main_node.fill_node(splitted_key, node.nodes);

        self.nodes.insert(base_key, Box::new(main_node));
    }

    fn fill_node(&mut self, splitted_key: &'a str, new_nodes: HashMap<&'a str, Box<Radyx<'a, T>>>) {
        let new_node = self.nodes.get_mut(splitted_key).unwrap();

        if new_node.nodes.is_empty() {
            new_node.nodes = new_nodes;
        }
    }
}

impl<T: Debug> Default for Radyx<'_, T> {
    fn default() -> Self {
        Self::new_empty()
    }
}
