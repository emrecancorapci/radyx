use std::collections::HashMap;

use crate::Radyx;

#[test]
fn basic() {
    let mut node: Radyx<'_, &str> = Radyx::default();

    node.insert("/home", "Home");
    node.insert("/home/more", "More than Home");

    assert_eq!(Some(&"Home"), node.get("/home"));
    assert_eq!(Some(&"More than Home"), node.get("/home/more"));
}

#[test]
fn two_leaf() {
    let mut node: Radyx<'_, &str> = Radyx::default();

    node.insert("/home", "Home");
    node.insert("/home/prim", "Prim");
    node.insert("/home/sec", "Sec");

    assert_eq!(Some(&"Prim"), node.get("/home/prim"));
    assert_eq!(Some(&"Sec"), node.get("/home/sec"));
}

#[test]
fn short_splitter_leaf() {
    let mut node: Radyx<'_, &str> = Radyx::default();

    node.insert("/home", "Home");
    node.insert("/home/mountain", "Mountain");
    node.insert("/home/mount", "Mount");

    assert_eq!(Some(&"Mountain"), node.get("/home/mountain"));
    assert_eq!(Some(&"Mount"), node.get("/home/mount"));
}

#[test]
fn long_splitting_leaf() {
    let mut node: Radyx<'_, &str> = Radyx::default();

    node.insert("/home", "Home");
    node.insert("/home/mount", "Mount");
    node.insert("/home/mountain", "Mountain");

    assert_eq!(&"Mount", node.get("/home/mount").unwrap());
    assert_eq!(&"Mountain", node.get("/home/mountain").unwrap());
}

#[test]
fn middle_splitting_leaf() {
    let mut node: Radyx<'_, &str> = Radyx::default();

    node.insert("/home", "Home");
    node.insert("/home/mountain", "Mountain");
    node.insert("/home/maintain", "Maintain");

    assert_eq!(Some(&"Mountain"), node.get("/home/mountain"));
    assert_eq!(Some(&"Maintain"), node.get("/home/maintain"));
}

#[test]
fn two_spliting_leaf() {
    let mut node: Radyx<'_, &str> = Radyx::default();

    node.insert("/home", "Home");
    node.insert("/home/maintaining", "Maintaining");
    node.insert("/home/maintainer", "Maintainer");
    node.insert("/home/main", "Main");

    assert_eq!(Some(&"Maintaining"), node.get("/home/maintaining"));
    assert_eq!(Some(&"Maintainer"), node.get("/home/maintainer"));
    assert_eq!(Some(&"Main"), node.get("/home/main"));
}

#[test]
fn fake_two_spliting_leaf() {
    let mut node: Radyx<'_, &str> = Radyx::default();

    node.insert("/home", "Home");
    node.insert("/home/maintaining", "Maintaining");
    node.insert("/home/maintainer", "Maintainer");
    node.insert("/home/main/", "Main");

    assert_eq!(Some(&"Maintaining"), node.get("/home/maintaining"));
    assert_eq!(Some(&"Maintainer"), node.get("/home/maintainer"));
    assert_eq!(Some(&"Main"), node.get("/home/main/"));
}

#[test]
fn random_splits() {
    let mut node: Radyx<'_, &str> = Radyx::default();

    node.insert("/home", "Home");
    node.insert("/home/maintaining", "Maintaining");
    node.insert("/home/maintainer", "Maintainer");
    node.insert("/home/main", "Main");
    node.insert("/home/master", "Master");
    node.insert("/hone/main", "Away From Home");

    assert_eq!(Some(&"Maintaining"), node.get("/home/maintaining"));
    assert_eq!(Some(&"Maintainer"), node.get("/home/maintainer"));
    assert_eq!(Some(&"Main"), node.get("/home/main"));
    assert_eq!(Some(&"Away From Home"), node.get("/hone/main"));
    assert_eq!(Some(&"Master"), node.get("/home/master"));
}

#[test]
fn overwrite_value() {
    let mut node: Radyx<'_, &str> = Radyx::default();

    node.insert("/home", "Home");
    node.insert("/home", "New Home"); // Overwrite value

    assert_eq!(Some(&"New Home"), node.get("/home")); // Ensure new value is stored
}

#[test]
fn empty_and_root_key() {
    let mut node: Radyx<'_, &str> = Radyx::default();

    node.insert("", "Root");
    node.insert("/", "Root Slash");

    // assert_eq!(Some(&"Root"), node.get(""));
    assert_eq!(Some(&"Root Slash"), node.get("/"));
}

#[test]
fn case_sensitivity() {
    let mut node: Radyx<'_, &str> = Radyx::default();

    node.insert("/home", "Home");
    node.insert("/Home", "Capital Home");

    assert_eq!(Some(&"Home"), node.get("/home"));
    assert_eq!(Some(&"Capital Home"), node.get("/Home"));
}

#[test]
fn non_existent_keys() {
    let mut node: Radyx<'_, &str> = Radyx::default();

    node.insert("/home", "Home");

    assert_eq!(None, node.get("/hom"));
    assert_eq!(None, node.get("/homer"));
    assert_eq!(None, node.get("/nonexistent"));
    assert_eq!(None, node.get("/home/missing"));
}

#[test]
fn large_scale_insertions() {
    let mut node: Radyx<'_, String> = Radyx::default();
    let mut value_map = HashMap::new();

    for i in 0..1000 {
        let k = format!("/key{}", i);
        let v = format!("Value{}", i);
        value_map.insert(k, v);
    }

    for (k, v) in value_map.iter() {
        node.insert(&k, v.to_string());
    }

    for i in 0..1000 {
        let key = format!("/key{}", i);
        assert_eq!(Some(&format!("Value{}", i)), node.get(&key));
    }

    assert_eq!(None, node.get("/key1001"));
}
