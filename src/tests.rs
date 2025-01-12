#[cfg(test)]
use crate::RadixNode;

#[test]
fn basic_usage() {
    let mut node: RadixNode<'_, String> = RadixNode::default();

    let _ = node.add("/home", Some(String::from("Home")));
    let __ = node.add("/home/more", Some(String::from("Not Home")));

    assert_eq!(&String::from("Home"), node.get("/home").unwrap());
}

#[test]
fn two_leaf() {
    let mut node: RadixNode<'_, String> = RadixNode::default();

    let _ = node.add("/home", Some(String::from("Home")));
    let __ = node.add("/home/prim", Some(String::from("Prim")));
    let ___ = node.add("/home/sec", Some(String::from("Sec")));

    assert_eq!(&String::from("Prim"), node.get("/home/prim").unwrap());
    assert_eq!(&String::from("Sec"), node.get("/home/sec").unwrap());
}

#[test]
fn short_splitting_leaf() {
    let mut node: RadixNode<'_, String> = RadixNode::default();

    let _ = node.add("/home", Some(String::from("Home")));
    let __ = node.add("/home/pasta", Some(String::from("Pasta")));
    let ___ = node.add("/home/pa", Some(String::from("Pa?")));

    assert_eq!(&String::from("Pasta"), node.get("/home/pasta").unwrap());
    assert_eq!(&String::from("Pa?"), node.get("/home/pa").unwrap());
}
