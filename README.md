# Radix Tree in Rust

A Rust implementation of a Radix Tree (also known as a Compact Prefix Tree) designed to be efficient, scalable, and easy to use. Radix Trees are widely used in applications such as autocomplete systems, routing tables, and prefix-based search.

## Installation

Add the following to your Cargo.toml:

```toml
[dependencies]
radix_tree = { git = "https://github.com/emrecancorapci/radix-tree.git" }
```

Then, include it in your project:

```rust
use radix_tree::RadixNode;
```

### Usage

```rust
use radix_tree::RadixNode;

let mut node: RadixNode<'_, String> = RadixNode::default();

let _ = node.insert("/home", String::from("Home"));
let _ = node.insert("/nothome", String::from("Not Home"));

assert_eq!(&String::from("Home"), node.get("/home").unwrap());
```
