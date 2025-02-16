# RADYX - A Basic Radix Tree Implementation

A Rust implementation of a Radix Tree (also known as a Compact Prefix Tree) designed to be efficient, scalable, and easy to use. Radix Trees are widely used in applications such as autocomplete systems, routing tables, and prefix-based search.

## Installation

Add the following to your Cargo.toml:

```toml
[dependencies]
radyx = "*"
```

or use the cargo cli

```bash
cargo add radyx
```

### Usage

```rust
use radyx::Radyx;

let mut node: Radyx<&str> = Radyx::default();

node.insert("/home", "Home");
node.insert("/nothome", "Elsewhere");

assert_eq!(Some(&"Home"), node.get("/home"));
```
