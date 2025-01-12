use crate::{find_common_index, RadixNode};

#[test]
fn basic_usage() {
    let mut node: RadixNode<'_, String> = RadixNode::default();

    let _ = node.add("/home", Some(String::from("Home")));
    let _ = node.add("/home/more", Some(String::from("Not Home")));

    assert_eq!(&String::from("Home"), node.get("/home").unwrap());
}

#[test]
fn two_leaf() {
    let mut node: RadixNode<'_, String> = RadixNode::default();

    let _ = node.add("/home", Some(String::from("Home")));
    let _ = node.add("/home/prim", Some(String::from("Prim")));
    let _ = node.add("/home/sec", Some(String::from("Sec")));

    assert_eq!(&String::from("Prim"), node.get("/home/prim").unwrap());
    assert_eq!(&String::from("Sec"), node.get("/home/sec").unwrap());
}

#[test]
fn short_splitter_leaf() {
    let mut node: RadixNode<'_, String> = RadixNode::default();

    let _ = node.add("/home", Some(String::from("Home")));
    let _ = node.add("/home/pasta", Some(String::from("Pasta")));
    let _ = node.add("/home/pa", Some(String::from("Pa?")));

    assert_eq!(&String::from("Pasta"), node.get("/home/pasta").unwrap());
    assert_eq!(&String::from("Pa?"), node.get("/home/pa").unwrap());
}

#[test]
fn long_splitting_leaf() {
    let mut node: RadixNode<'_, String> = RadixNode::default();

    let _ = node.add("/home", Some(String::from("Home")));
    let _ = node.add("/home/pasta", Some(String::from("Pasta")));
    let _ = node.add("/home/pastafarian", Some(String::from("Pastafarian")));

    assert_eq!(&String::from("Pasta"), node.get("/home/pasta").unwrap());
    assert_eq!(&String::from("Pastafarian"), node.get("/home/pastafarian").unwrap());
}

#[test]
fn middle_splitting_leaf() {
    let mut node: RadixNode<'_, String> = RadixNode::default();

    let _ = node.add("/home", Some(String::from("Home")));
    let _ = node.add("/home/pasta", Some(String::from("Pasta")));
    let _ = node.add("/home/party", Some(String::from("Party")));

    assert_eq!(&String::from("Pasta"), node.get("/home/pasta").unwrap());
    assert_eq!(&String::from("Party"), node.get("/home/party").unwrap());
}

#[test]
fn two_spliting_leaf() {
    let mut node: RadixNode<'_, String> = RadixNode::default();

    assert_eq!(Ok(()), node.add("/home", Some(String::from("Home"))));
    assert_eq!(Ok(()), node.add("/home/pasta", Some(String::from("Pasta"))));
    assert_eq!(Ok(()), node.add("/home/pastry", Some(String::from("Pastry"))));
    assert_eq!(Ok(()), node.add("/home/pa", Some(String::from("Pa"))));

    assert_eq!(Some(&String::from("Pasta")), node.get("/home/pasta"));
    assert_eq!(Some(&String::from("Pastry")), node.get("/home/pastry"));
    assert_eq!(Some(&String::from("Pa")), node.get("/home/pa"));
}

#[test]
fn random_splits() {
    let mut node: RadixNode<'_, String> = RadixNode::default();

    assert_eq!(Ok(()), node.add("/home", Some(String::from("Home"))));
    assert_eq!(Ok(()), node.add("/home/pasta", Some(String::from("Pasta"))));
    assert_eq!(Ok(()), node.add("/home/party", Some(String::from("Party"))));
    assert_eq!(Ok(()), node.add("/home/passive", Some(String::from("Passive"))));
    assert_eq!(Ok(()), node.add("/home/pa", Some(String::from("Pa"))));
    assert_eq!(Ok(()), node.add("/hone/pa", Some(String::from("Away From Pa"))));


    assert_eq!(Some(&String::from("Pasta")), node.get("/home/pasta"));
    assert_eq!(Some(&String::from("Party")), node.get("/home/party"));
    assert_eq!(Some(&String::from("Pa")), node.get("/home/pa"));
    assert_eq!(Some(&String::from("Away From Pa")), node.get("/home/pa"));
    assert_eq!(Some(&String::from("Passive")), node.get("/home/passive"));
    
}

#[test]
fn fci_commutativity() {
    let str1 = "/Pasta";
    let str2 = "/Pasanger";
    let str3 = "/s";
    let str4 = "/f";

    assert_eq!(find_common_index(str2, str1), find_common_index(str1, str2));
    assert_eq!(find_common_index(str3, str4), find_common_index(str4, str3));
}
