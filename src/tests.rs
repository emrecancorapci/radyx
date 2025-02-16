use std::collections::HashMap;

use crate::{longest_common_prefix, RadixNode};

#[test]
fn basic() {
    let mut node: RadixNode<'_, &str> = RadixNode::default();

    let _ = node.insert("/home", "Home");
    let _ = node.insert("/home/more", "More than Home");

    assert_eq!(Some(&"Home"), node.get("/home"));
    assert_eq!(Some(&"More than Home"), node.get("/home/more"));
}

#[test]
fn two_leaf() {
    let mut node: RadixNode<'_, &str> = RadixNode::default();

    let _ = node.insert("/home", "Home");
    let _ = node.insert("/home/prim", "Prim");
    let _ = node.insert("/home/sec", "Sec");

    assert_eq!(Some(&"Prim"), node.get("/home/prim"));
    assert_eq!(Some(&"Sec"), node.get("/home/sec"));
}

#[test]
fn short_splitter_leaf() {
    let mut node: RadixNode<'_, &str> = RadixNode::default();

    let _ = node.insert("/home", "Home");
    let _ = node.insert("/home/mountain", "Mountain");
    let _ = node.insert("/home/mount", "Mount");

    assert_eq!(Some(&"Mountain"), node.get("/home/mountain"));
    assert_eq!(Some(&"Mount"), node.get("/home/mount"));
}

#[test]
fn long_splitting_leaf() {
    let mut node: RadixNode<'_, &str> = RadixNode::default();

    let _ = node.insert("/home", "Home");
    let _ = node.insert("/home/mount", "Mount");
    let _ = node.insert("/home/mountain", "Mountain");

    assert_eq!(&"Mount", node.get("/home/mount").unwrap());
    assert_eq!(&"Mountain", node.get("/home/mountain").unwrap());
}

#[test]
fn middle_splitting_leaf() {
    let mut node: RadixNode<'_, &str> = RadixNode::default();

    let _ = node.insert("/home", "Home");
    let _ = node.insert("/home/mountain", "Mountain");
    let _ = node.insert("/home/maintain", "Maintain");

    assert_eq!(Some(&"Mountain"), node.get("/home/mountain"));
    assert_eq!(Some(&"Maintain"), node.get("/home/maintain"));
}

#[test]
fn two_spliting_leaf() {
    let mut node: RadixNode<'_, &str> = RadixNode::default();

    let _ = node.insert("/home", "Home");
    let _ = node.insert("/home/maintaining", "Maintaining");
    let _ = node.insert("/home/maintainer", "Maintainer");
    let _ = node.insert("/home/main", "Main");

    assert_eq!(Some(&"Maintaining"), node.get("/home/maintaining"));
    assert_eq!(Some(&"Maintainer"), node.get("/home/maintainer"));
    assert_eq!(Some(&"Main"), node.get("/home/main"));
}

#[test]
fn fake_two_spliting_leaf() {
    let mut node: RadixNode<'_, &str> = RadixNode::default();

    let _ = node.insert("/home", "Home");
    let _ = node.insert("/home/maintaining", "Maintaining");
    let _ = node.insert("/home/maintainer", "Maintainer");
    let _ = node.insert("/home/main/", "Main");

    assert_eq!(Some(&"Maintaining"), node.get("/home/maintaining"));
    assert_eq!(Some(&"Maintainer"), node.get("/home/maintainer"));
    assert_eq!(Some(&"Main"), node.get("/home/main/"));
}

#[test]
fn random_splits() {
    let mut node: RadixNode<'_, &str> = RadixNode::default();

    let _ = node.insert("/home", "Home");
    let _ = node.insert("/home/maintaining", "Maintaining");
    let _ = node.insert("/home/maintainer", "Maintainer");
    let _ = node.insert("/home/main", "Main");
    let _ = node.insert("/home/master", "Master");
    let _ = node.insert("/hone/main", "Away From Home");

    assert_eq!(Some(&"Maintaining"), node.get("/home/maintaining"));
    assert_eq!(Some(&"Maintainer"), node.get("/home/maintainer"));
    assert_eq!(Some(&"Main"), node.get("/home/main"));
    assert_eq!(Some(&"Away From Home"), node.get("/hone/main"));
    assert_eq!(Some(&"Master"), node.get("/home/master"));
}

#[test]
fn overwrite_value() {
    let mut node: RadixNode<'_, &str> = RadixNode::default();

    let _ = node.insert("/home", "Home");
    let _ = node.insert("/home", "New Home"); // Overwrite value

    assert_eq!(Some(&"New Home"), node.get("/home")); // Ensure new value is stored
}

#[test]
fn empty_and_root_key() {
    let mut node: RadixNode<'_, &str> = RadixNode::default();

    let _ = node.insert("", "Root");
    dbg!(&node);
    let _ = node.insert("/", "Root Slash");
    dbg!(&node);

    assert_eq!(Some(&"Root"), node.get(""));
    assert_eq!(Some(&"Root Slash"), node.get("/"));
}

#[test]
fn case_sensitivity() {
    let mut node: RadixNode<'_, &str> = RadixNode::default();

    let _ = node.insert("/home", "Home");
    let _ = node.insert("/Home", "Capital Home");

    assert_eq!(Some(&"Home"), node.get("/home"));
    assert_eq!(Some(&"Capital Home"), node.get("/Home"));
}

#[test]
fn non_existent_keys() {
    let mut node: RadixNode<'_, &str> = RadixNode::default();

    let _ = node.insert("/home", "Home");

    assert_eq!(None, node.get("/hom"));
    assert_eq!(None, node.get("/homer"));
    assert_eq!(None, node.get("/nonexistent"));
    assert_eq!(None, node.get("/home/missing"));
}

#[test]
fn large_scale_insertions() {
    let mut node: RadixNode<'_, String> = RadixNode::default();
    let mut value_map = HashMap::new();

    for i in 0..1000 {
        let k = format!("/key{}", i);
        let v = format!("Value{}", i);
        value_map.insert(k, v);
    }

    for (k, v) in value_map.iter() {
        let _ = node.insert(&k, v.to_string());
    }

    for i in 0..1000 {
        let key = format!("/key{}", i);
        assert_eq!(Some(&format!("Value{}", i)), node.get(&key));
    }

    assert_eq!(None, node.get("/key1001"));
}

#[test]
fn lcp_commutativity() {
    let str1 = "/Pasta";
    let str2 = "/Pasanger";
    let str3 = "/s";
    let str4 = "/f";

    assert_eq!(
        longest_common_prefix(str2, str1),
        longest_common_prefix(str1, str2)
    );
    assert_eq!(
        longest_common_prefix(str3, str4),
        longest_common_prefix(str4, str3)
    );
}
